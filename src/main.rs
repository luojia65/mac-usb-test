mod ffi;

use core::mem::MaybeUninit;
use std::ffi::CStr;
use ffi::*;

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
        &mut iter as *mut _ as *mut _
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
            return;
        }
        let name = unsafe { CStr::from_ptr(name.as_ptr()) };
        dbg!(name);
        unsafe { IOObjectRelease(next) };
    }

    unsafe { IOObjectRelease(iter) };
}
