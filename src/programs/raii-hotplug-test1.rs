mod ffi;
use ffi::*;

use core::mem::MaybeUninit;
use core::mem::transmute;
use core::ffi::c_void;
use core::pin::Pin;
use core::fmt;
use core::task::{Poll, Context, Waker};
use std::ffi::CStr;
use std::io;
use mach::kern_return;
use mach::mach_port::*;
use mach::traps::mach_task_self;
use mach::port::mach_port_t;
use core_foundation::uuid::CFUUIDGetUUIDBytes;
use core_foundation::runloop::*;
use futures::prelude::*;
use futures::stream::StreamExt;

#[derive(Debug)]
pub enum Error {
    CreateMasterPort(io::Error),
    CreateDictionary,
    AddNotification(io::Error),
}

pub type Result<T> = core::result::Result<T, Error>;

pub fn devices() -> Result<Box<Devices>> {
    Devices::new_usb()
}

pub struct Devices {
    notify_port: IONotificationPortRef,
    last_device: Option<Box<MyDevice>>,
    waker: Option<Waker>,
}

unsafe impl Send for Devices {}
unsafe impl Sync for Devices {}

impl Devices {
    fn new_usb() -> Result<Box<Self>> {
        let mut master_port = MaybeUninit::uninit();
        let kr = unsafe { IOMasterPort(MACH_PORT_NULL, master_port.as_mut_ptr()) };
        if kr != kern_return::KERN_SUCCESS {
            return Err(Error::CreateMasterPort(io::Error::from_raw_os_error(kr)));
        }
        let master_port: mach_port_t = unsafe { master_port.assume_init() };
    
        let matching_dict = unsafe { IOServiceMatching(kIOUSBDeviceClassName()) };
        if matching_dict == core::ptr::null_mut() {
            return Err(Error::CreateDictionary);
        }
    
        let notify_port: IONotificationPortRef = unsafe { IONotificationPortCreate(master_port) };
        let runloop_source = unsafe { IONotificationPortGetRunLoopSource(notify_port) };
    
        let runloop = unsafe { CFRunLoopGetCurrent() };
        unsafe { CFRunLoopAddSource(runloop, runloop_source, kCFRunLoopDefaultMode) };
        
        let ans: Box<Devices> = Box::new(Devices {
            notify_port,
            last_device: None,
            waker: None,
        });
        let ans_ptr = Box::into_raw(ans);
        let mut added_iter = MaybeUninit::uninit();
        let kr = unsafe { IOServiceAddMatchingNotification(
            notify_port,
            kIOFirstMatchNotification(),
            matching_dict,
            device_added,
            ans_ptr as *const _, // refCon
            added_iter.as_mut_ptr(),
        ) };
        let added_iter: io_iterator_t = unsafe { added_iter.assume_init() };
        if kr != kern_return::KERN_SUCCESS {
            return Err(Error::AddNotification(io::Error::from_raw_os_error(kr)));
        }

        device_added(ans_ptr as *const _, added_iter);

        unsafe { &mut *ans_ptr }.last_device = None;

        unsafe { mach_port_deallocate(mach_task_self(), master_port) };
        
        let ans = unsafe { Box::from_raw(ans_ptr) };
        Ok(ans)
    }

    pub fn run() -> ! {
        unsafe { CFRunLoopRun() };
        loop {} // never reach this point
    }
}

impl Stream for Devices {
    type Item = Box<MyDevice>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        self.waker = Some(cx.waker().clone());
        if let Some(device) = self.last_device.take() {
            Poll::Ready(Some(device))
        } else {
            Poll::Pending
        }
    }
}

pub struct MyDevice {
    notification: io_object_t,
    device_interface: *mut *mut IOUSBDeviceInterface942,
    device_name: [i8; 128],
    location_id: u32,
}

impl MyDevice {
    fn new() -> Box<MyDevice> {
        Box::new(unsafe { MaybeUninit::uninit().assume_init() })
    }

    fn debug_name(&self) -> &CStr {
        &unsafe { CStr::from_ptr(self.device_name.as_ptr()) }
    }
}

impl fmt::Debug for MyDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MyDevice")
            .field("notification", &self.notification)
            .field("device_interface", &self.device_interface)
            .field("device_name", &self.debug_name())
            .field("location_id", &self.location_id)
            .finish()  
    }
}

extern "C" fn device_added (
    ref_con: *const c_void,
    iterator: io_iterator_t,
) {
    let mut devices: Pin<&mut Devices> = unsafe { 
        Pin::new_unchecked(Box::leak(Box::from_raw(ref_con as *mut Devices))) 
    };
    let notify_port = devices.notify_port;
    loop {
        let usb_device = unsafe { IOIteratorNext(iterator) };
        if usb_device == 0 {
            break;
        }
        println!("Device 0x{:08x} added!", usb_device);
        
        let mut device = MyDevice::new();
        let kr = unsafe { IORegistryEntryGetName(
            usb_device, 
            &mut device.device_name as *mut _ as *mut _
        ) };
        if kr != kern_return::KERN_SUCCESS {
            device.device_name[0] = unsafe { transmute(b'\0') };
        }
        let name = unsafe { CStr::from_ptr(device.device_name.as_ptr()) };
        println!("Device name: {:?}", name);
        
        let mut plugin_interface = MaybeUninit::uninit();
        let mut score = MaybeUninit::uninit();
        let kr = unsafe {
            IOCreatePlugInInterfaceForService(
                usb_device,
                kIOUSBDeviceUserClientTypeID(),
                kIOCFPlugInInterfaceID(),
                plugin_interface.as_mut_ptr(),
                score.as_mut_ptr(),
            )
        };
        if kr != kern_return::KERN_SUCCESS {
            println!("Unable to create a plugin! {:08x}", kr);
            continue;
        }
        let plugin_interface = unsafe { plugin_interface.assume_init() };
        let score = unsafe { score.assume_init() };
        println!("Score: {}", score);
        
        let res = unsafe { 
            ((**plugin_interface).QueryInterface)(
                plugin_interface,
                CFUUIDGetUUIDBytes(kIOUSBDeviceInterfaceID942()),
                &mut device.device_interface as *mut _ as *mut _,
            )
        };
        unsafe { IODestroyPlugInInterface(plugin_interface) };
        if res != 0 {
            println!("QueryInterface not success! 0x{:08X}", kr);
            continue;
        }
        
        let mut location_id = MaybeUninit::uninit();
        let kr = unsafe {
            ((**device.device_interface).GetLocationID)(
                device.device_interface,
                location_id.as_mut_ptr(),
            )
        };
        if kr != mach::kern_return::KERN_SUCCESS {
            println!("GetLocationID not success! 0x{:08X}", kr);
            continue;
        }
        let location_id = unsafe { location_id.assume_init() };
        device.location_id = location_id;

        let notification = &device.notification as *const _ as  *mut _;
        let device_ptr = Box::into_raw(device);
        let kr = unsafe { IOServiceAddInterestNotification(
            notify_port,
            usb_device,
            kIOGeneralInterest(),
            device_notify,
            device_ptr as *const _,
            notification
        ) };
        if kr != kern_return::KERN_SUCCESS {
            println!("IOServiceAddInterestNotification returned 0x{:08x}", kr);
        }
        unsafe { IOObjectRelease(usb_device) };

        devices.last_device = Some(unsafe { Box::from_raw(device_ptr )});
        if let Some(waker) = devices.waker.clone() {
            waker.wake();
        }
    }
}

extern "C" fn device_notify(
    ref_con: *const c_void, 
    service: io_service_t,
    message_type: u32,
    message_argument: *const c_void,
) {
    let _ = message_argument;
    let device = unsafe { Box::from_raw(ref_con as *const _ as *mut MyDevice) };
    println!("Device {:?} received message 0x{:08x}", device.debug_name(), message_type);
    if message_type == kIOMessageServiceIsTerminated {
        println!("Device 0x{:08x} removed!", service);
        
        println!("Device was at location: {}", device.location_id);

        unsafe { 
            ((**device.device_interface).Release) (device.device_interface)
        };

        unsafe { IOObjectRelease(device.notification) };
        // Box<MyDevice> is freed here
    } else {
        Box::leak(device);
    }
}

#[async_std::main]
async fn main() -> Result<()> {
    let mut devices = devices()?;
    async_std::task::spawn(async move { 
        while let Some(device) = devices.next().await { 
            println!("{:?}", device);
        } });
    Devices::run()
}
