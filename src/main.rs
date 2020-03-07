mod ffi;

use core::mem::MaybeUninit;
use core_foundation::uuid::CFUUIDGetUUIDBytes;
use ffi::*;
use std::ffi::CStr;

fn main() {
    let matching_dict = unsafe { IOServiceMatching(kIOUSBDeviceClassName()) };
    dbg!(matching_dict);
    if matching_dict == core::ptr::null_mut() {
        println!("IOServiceMatching returned NULL.");
        return;
    }

    let mut iter: MaybeUninit<io_iterator_t> = MaybeUninit::uninit();
    let kr = unsafe {
        IOServiceGetMatchingServices(kIOMasterPortDefault, matching_dict, iter.as_mut_ptr())
    };
    dbg!(kr);
    if kr != mach::kern_return::KERN_SUCCESS {
        println!("IOServiceGetMatchingServices not success!");
        return;
    }
    let iter = unsafe { iter.assume_init() };
    dbg!(iter);

    my_get_usb_interface(iter);

    unsafe { IOObjectRelease(iter) };
}

fn my_get_usb_interface(iter: io_iterator_t) {
    loop {
        let service = unsafe { IOIteratorNext(iter) };
        if service == 0 {
            break;
        }
        dbg!(service);

        // Create an intermediate plug-in
        let mut plugin_interface = MaybeUninit::uninit();
        let mut score = MaybeUninit::uninit();
        let kr = unsafe {
            IOCreatePlugInInterfaceForService(
                service,
                kIOUSBDeviceUserClientTypeID(),
                kIOCFPlugInInterfaceID(),
                plugin_interface.as_mut_ptr(),
                score.as_mut_ptr(),
            )
        };
        // Don't need the device object after intermediate plug-in is created
        unsafe { IOObjectRelease(service) };
        if kr != mach::kern_return::KERN_SUCCESS {
            println!("IOCreatePlugInInterfaceForService not success! {}", kr);
            unsafe { IOObjectRelease(service) };
            continue;
        }
        dbg!(unsafe { score.assume_init() });

        // 
        let mut name = [0i8; 128];
        let kr = unsafe { IORegistryEntryGetName(service, name.as_mut_ptr()) };
        if kr != mach::kern_return::KERN_SUCCESS {
            println!("IORegistryEntryGetName not success!");
            continue;
        }
        let name = unsafe { CStr::from_ptr(name.as_ptr()) };
        dbg!(name);

        // Now create the device interface
        let plugin_interface = unsafe { plugin_interface.assume_init() };
        let mut device_interface = MaybeUninit::<*mut *mut IOUSBDeviceInterface942>::uninit();
        let kr = unsafe {
            ((**plugin_interface).QueryInterface)(
                plugin_interface,
                CFUUIDGetUUIDBytes(kIOUSBDeviceInterfaceID942()),
                device_interface.as_mut_ptr().cast(),
            )
        };
        //Don't need the device object after intermediate plug-in is created
        unsafe { IODestroyPlugInInterface(plugin_interface) };
        if kr != mach::kern_return::KERN_SUCCESS {
            println!("QueryInterface not success! {}", kr);
            continue;
        }
        let device_interface = unsafe { device_interface.assume_init() };
        let mut location_id = MaybeUninit::uninit();
        let kr = unsafe {
            ((**device_interface).GetLocationID)(
                device_interface,
                location_id.as_mut_ptr()
            )
        };
        if kr != mach::kern_return::KERN_SUCCESS {
            println!("GetLocationID not success! {}", kr);
            continue;
        }
        let location_id = unsafe { location_id.assume_init() };
        dbg!(location_id);

        let mut usb_device_address = MaybeUninit::uninit();
        let kr = unsafe {
            ((**device_interface).GetDeviceAddress)(device_interface, usb_device_address.as_mut_ptr())
        };
        if kr != mach::kern_return::KERN_SUCCESS {
            println!("GetDeviceAddress not success! {}", kr);
            continue;
        }
        let usb_device_address = unsafe { usb_device_address.assume_init() };
        dbg!(usb_device_address);


        unsafe { ((**plugin_interface).Release)(plugin_interface) };
    }
}
