use libc::{c_char, c_void};
use mach::kern_return;

extern "C" {
    pub static kIOMasterPortDefault: mach_port_t;
    pub fn IOServiceMatching(name: *const c_char) -> CFMutableDictionaryRef;
    pub fn IOServiceGetMatchingServices(
        master_port: mach_port_t,
        matching: CFDictionaryRef,
        existing: *mut io_iterator_t,
    ) -> kern_return::kern_return_t;
    pub fn IOIteratorNext(iterator: io_iterator_t) -> io_object_t;
    pub fn IOObjectRelease(object: io_object_t) -> kern_return::kern_return_t;
    pub fn IORegistryEntryGetName(
        entry: io_registry_entry_t,
        name: *mut c_char,
    ) -> kern_return::kern_return_t;
}

#[allow(non_camel_case_types)]
pub type mach_port_t = u32;
#[allow(non_camel_case_types)]
pub type io_object_t = mach_port_t;
#[allow(non_camel_case_types)]
pub type io_iterator_t = io_object_t;
#[allow(non_camel_case_types)]
pub type io_registry_entry_t = io_object_t;

#[allow(non_snake_case)]
pub fn kIOUSBDeviceClassName() -> *const c_char {
    b"IOUSBDevice\0".as_ptr() as *const c_char
}

#[repr(C)]
pub struct __CFDictionary {
    __private: c_void,
}
pub type CFMutableDictionaryRef = *mut __CFDictionary;
pub type CFDictionaryRef = *const __CFDictionary;
