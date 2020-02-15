use libc::{c_char, c_void};
use mach::kern_return;
use core_foundation::{
    base::{CFAllocatorRef},
    uuid::{CFUUIDRef, CFUUIDBytes}
};
use core::ptr;

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
    pub fn IOCreatePlugInInterfaceForService(
        service: io_service_t,
        plugin_type: CFUUIDRef,
        interface_type: CFUUIDRef,
        the_interface: *mut *mut *mut IOCFPlugInInterface,
        the_score: *mut i32,
    ) -> kern_return::kern_return_t;
    pub fn CFUUIDGetConstantUUIDWithBytes(
        alloc: CFAllocatorRef,
        byte0: UInt8, byte1: UInt8, byte2: UInt8, byte3: UInt8, 
        byte4: UInt8, byte5: UInt8, byte6: UInt8, byte7: UInt8, 
        byte8: UInt8, byte9: UInt8, byte10: UInt8, byte11: UInt8, 
        byte12: UInt8, byte13: UInt8, byte14: UInt8, byte15: UInt8
    ) -> CFUUIDRef;
}

#[allow(non_camel_case_types)]
pub type mach_port_t = u32;
#[allow(non_camel_case_types)]
pub type io_object_t = mach_port_t;
#[allow(non_camel_case_types)]
pub type io_iterator_t = io_object_t;
#[allow(non_camel_case_types)]
pub type io_registry_entry_t = io_object_t;
#[allow(non_camel_case_types)]
pub type io_service_t = io_object_t;

#[allow(non_snake_case)]
pub fn kIOUSBDeviceClassName() -> *const c_char {
    b"IOUSBDevice\0".as_ptr() as *const c_char
}
#[allow(non_snake_case)]
pub fn kIOUSBDeviceUserClientTypeID() -> CFUUIDRef {
    unsafe { CFUUIDGetConstantUUIDWithBytes(ptr::null(), 
        0x9d, 0xc7, 0xb7, 0x80, 0x9e, 0xc0, 0x11, 0xD4,
        0xa5, 0x4f, 0x00, 0x0a, 0x27, 0x05, 0x28, 0x61
    ) }
}
#[allow(non_snake_case)]
pub fn kIOCFPlugInInterfaceID() -> CFUUIDRef {
    unsafe { CFUUIDGetConstantUUIDWithBytes(ptr::null(),     
        0xC2, 0x44, 0xE8, 0x58, 0x10, 0x9C, 0x11, 0xD4,
        0x91, 0xD4, 0x00, 0x50, 0xE4, 0xC6, 0x42, 0x6F
    ) }
} 
// macOS 10.14 & up
#[allow(non_snake_case)] 
pub fn kIOUSBInterfaceInterfaceID942() -> CFUUIDRef {
    unsafe { CFUUIDGetConstantUUIDWithBytes(ptr::null(),     
        0x87, 0x52, 0x66, 0x3B, 0xC0, 0x7B, 0x4B, 0xAE,
        0x95, 0x84, 0x22, 0x03, 0x2F, 0xAB, 0x9C, 0x5A
    ) }
} 

#[repr(C)]
pub struct __CFDictionary {
    __private: c_void,
}
pub type CFMutableDictionaryRef = *mut __CFDictionary;
pub type CFDictionaryRef = *const __CFDictionary;

pub type UInt8 = u8;
pub type UInt16 = u16;
pub type UInt32 = u32;
pub type SInt32 = i32;
pub type REFIID = CFUUIDBytes;
pub type LPVOID = *const c_void;
pub type HRESULT = SInt32;
pub type ULONG = UInt32;
pub type IOReturn = kern_return::kern_return_t;

#[repr(C)]
#[allow(non_snake_case)]
pub struct IOCFPlugInInterface {
    // IUNKNOWN_C_GUTS
    pub _reserved: *const Self,
    pub QueryInterface: extern "C" fn(
        this: *mut *mut Self, // todo: verify
        iid: REFIID, 
        ppv: *mut LPVOID
    ) -> HRESULT,
    pub AddRef: extern "C" fn(this: *mut Self) -> ULONG,
    pub Release: extern "C" fn(this: *mut *mut Self) -> ULONG,
    // IOCFPLUGINBASE
    pub version: UInt16,
    pub revision: UInt16,
    pub Probe: extern "C" fn(
        this: *mut Self, 
        property_table: CFDictionaryRef, 
        service: io_service_t,
        order: *mut SInt32
    ) -> IOReturn,
    pub Start: extern "C" fn(
        this: *mut Self,
        property_table: CFDictionaryRef, 
        service: io_service_t
    ) -> IOReturn,
    pub Stop: extern "C" fn(this: *mut Self) -> IOReturn,
}
