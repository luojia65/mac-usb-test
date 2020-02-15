mod ffi;

use core::mem::MaybeUninit;
use std::ffi::CStr;
use ffi::*;
use core_foundation::uuid::CFUUIDGetUUIDBytes;

fn main() {
    let classes_to_match = unsafe { IOServiceMatching(kIOUSBDeviceClassName()) };
    dbg!(classes_to_match);
    if classes_to_match == core::ptr::null_mut() {
        println!("IOServiceMatching returned NULL.");
        return;
    }

    let mut iter: MaybeUninit<io_iterator_t> = MaybeUninit::uninit();
    let kr = unsafe { IOServiceGetMatchingServices(
        kIOMasterPortDefault,
        classes_to_match,
        iter.as_mut_ptr()
    ) };
    dbg!(kr);
    if kr != mach::kern_return::KERN_SUCCESS {
        println!("IOServiceGetMatchingServices not success!");
        return;
    }
    let iter = unsafe { iter.assume_init() };
    dbg!(iter);

    loop {
        let next = unsafe { IOIteratorNext(iter) };
        if next == 0 {
            break
        }
        dbg!(next);

        let mut name = vec![0i8; 1024];
        let kr = unsafe { IORegistryEntryGetName(
            next, 
            name.as_mut_ptr()
        ) };
        if kr != mach::kern_return::KERN_SUCCESS {
            println!("IORegistryEntryGetName not success!");
            continue;
        }
        let name = unsafe { CStr::from_ptr(name.as_ptr()) };
        dbg!(name);

        let mut plugin_interface = MaybeUninit::uninit();
        let mut score = MaybeUninit::uninit();
        let kr = unsafe { IOCreatePlugInInterfaceForService(
            next,
            kIOUSBDeviceUserClientTypeID(),
            kIOCFPlugInInterfaceID(),
            plugin_interface.as_mut_ptr(),
            score.as_mut_ptr(),
        ) };
        if kr != mach::kern_return::KERN_SUCCESS {
            println!("IOCreatePlugInInterfaceForService not success! {}", kr);
            unsafe { IOObjectRelease(next) };
            continue;
        } 
        dbg!(unsafe { score.assume_init() });

        let plugin_interface = unsafe { plugin_interface.assume_init() };
        let mut device = MaybeUninit::uninit();
        unsafe { 
            ((**plugin_interface).QueryInterface)(
                plugin_interface, 
                CFUUIDGetUUIDBytes(kIOUSBInterfaceInterfaceID942()), 
                device.as_mut_ptr()
            );
        }
        dbg!(unsafe { device.assume_init() });
        unsafe {
            ((**plugin_interface).Release)(plugin_interface)
        };

        unsafe { IOObjectRelease(next) };
    }

    unsafe { IOObjectRelease(iter) };
}
