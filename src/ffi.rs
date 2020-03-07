#![allow(unused)]
use core::ptr;
use core_foundation::{
    base::CFAllocatorRef,
    uuid::{CFUUIDBytes, CFUUIDRef},
};
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
        name: io_name_t,
    ) -> kern_return::kern_return_t;
    pub fn IOCreatePlugInInterfaceForService(
        service: io_service_t,
        plugin_type: CFUUIDRef,
        interface_type: CFUUIDRef,
        the_interface: *mut *mut *mut IOCFPlugInInterface,
        the_score: *mut i32,
    ) -> kern_return::kern_return_t;
    pub fn IODestroyPlugInInterface(
        interface: *mut *mut IOCFPlugInInterface,
    ) -> kern_return::kern_return_t;
    pub fn CFUUIDGetConstantUUIDWithBytes(
        alloc: CFAllocatorRef,
        byte0: UInt8,
        byte1: UInt8,
        byte2: UInt8,
        byte3: UInt8,
        byte4: UInt8,
        byte5: UInt8,
        byte6: UInt8,
        byte7: UInt8,
        byte8: UInt8,
        byte9: UInt8,
        byte10: UInt8,
        byte11: UInt8,
        byte12: UInt8,
        byte13: UInt8,
        byte14: UInt8,
        byte15: UInt8,
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
#[allow(non_camel_case_types)]
pub type io_name_t = *mut c_char; // [c_char; 128]

#[allow(non_snake_case)]
pub fn kIOUSBDeviceClassName() -> *const c_char {
    b"IOUSBDevice\0".as_ptr() as *const c_char
}
#[allow(non_snake_case)]
pub fn kIOUSBInterfaceClassName() -> *const c_char {
    b"IOUSBInterface\0".as_ptr() as *const c_char
}
#[allow(non_snake_case)]
pub fn kIOUSBHostDeviceClassName() -> *const c_char {
    b"IOUSBHostDevice\0".as_ptr() as *const c_char
}
#[allow(non_snake_case)]
pub fn kIOUSBHostInterfaceClassName() -> *const c_char {
    b"IOUSBHostInterface\0".as_ptr() as *const c_char
}
#[allow(non_snake_case)]
pub fn kIOUSBDeviceUserClientTypeID() -> CFUUIDRef {
    unsafe { CFUUIDGetConstantUUIDWithBytes(ptr::null(),
        0x9d, 0xc7, 0xb7, 0x80, 0x9e, 0xc0, 0x11, 0xD4,
        0xa5, 0x4f, 0x00, 0x0a, 0x27, 0x05, 0x28, 0x61,
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
#[allow(non_snake_case)]
pub fn kIOUSBDeviceInterfaceID942() -> CFUUIDRef {
    unsafe { CFUUIDGetConstantUUIDWithBytes(ptr::null(),
        0x56, 0xAD, 0x08, 0x9D, 0x87, 0x8D, 0x4B, 0xEA,
        0xA1, 0xF5, 0x2C, 0x8D, 0xC4, 0x3E, 0x8A, 0x98
    ) }
} 

#[repr(C)]
pub struct __CFDictionary {
    __private: c_void,
}
#[repr(C)]
pub struct __CFRunLoopSource {
    __private: c_void,
}
pub type CFMutableDictionaryRef = *mut __CFDictionary;
pub type CFDictionaryRef = *const __CFDictionary;
pub type CFRunLoopSourceRef = *mut __CFRunLoopSource;

pub type UInt8 = u8;
pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;
pub type SInt32 = i32;
pub type Boolean = bool;
pub type REFIID = CFUUIDBytes;
pub type LPVOID = *const c_void;
pub type HRESULT = SInt32;
pub type ULONG = UInt32;
pub type IOReturn = kern_return::kern_return_t;
pub type USBDeviceAddress = UInt16;
pub type AbsoluteTime = UInt64;

#[repr(C)]
#[allow(non_snake_case)]
pub struct IOUSBConfigurationDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub wTotalLength: u16,
    pub bNumInterfaces: u8,
    pub bConfigurationValue: u8,
    pub iConfiguration: u8,
    pub bmAttributes: u8,
    pub MaxPower: u8,
}

pub type IOUSBConfigurationDescriptorPtr = *mut IOUSBConfigurationDescriptor;

#[repr(C)]
#[allow(non_snake_case)]
pub struct IOUSBDevRequest {
    pub bmRequestType: u8,
    pub bRequest: u8,
    pub wValue: u16,
    pub wIndex: u16,
    pub wLength: u16,
    pub pData: *const c_void,
    pub wLenDone: u32,
}

pub type IOUSBDeviceRequestPtr = *mut IOUSBDevRequest;

#[cfg(target_endian = "big")]
#[repr(C)]
#[allow(non_snake_case)]
pub struct NumVersion {
    pub majorRev: u8,
    pub minorAndBugRev: u8,
    pub stage: u8,
    pub nonRelRev: u8,
}

#[cfg(target_endian = "little")]
#[repr(C)]
#[allow(non_snake_case)]
pub struct NumVersion {
    pub nonRelRev: u8,
    pub stage: u8,
    pub minorAndBugRev: u8,
    pub majorRev: u8,
}

pub type IOAsyncCallback1 = fn(refcon: *const c_void, result: IOReturn, arg0: *const c_void);

#[repr(C)]
#[allow(non_snake_case)]
pub struct IOUSBDevRequestTO {
    /// Request type: kUSBStandard, kUSBClass or kUSBVendor
    pub bmRequestType: u8,
    /// Request code
    pub bRequest: u8,
    /// 16 bit parameter for request, host endianess
    pub wValue: u16,
    /// 16 bit parameter for request, host endianess
    pub wIndex: u16,
    /// Length of data part of request, 16 bits, host endianess
    pub wLength: u16,
    /// Pointer to data for request - data returned in bus endianess
    pub pData: *const c_void,
    /// Set by standard completion routine to number of data bytes actually transferred
    pub wLenDone: u32,
    /// Specifies a time value in milliseconds. Once the request is queued on the bus, 
    /// if no data is transferred in this amount of time, the request will be aborted and returned.
    pub noDataTimeout: u32,
    /// Specifies a time value in milliseconds. Once the request is queued on the bus, 
    /// if the entire request is not completed in this amount of time, the request will be aborted and
    /// returned.
    pub completionTimeout: u32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct IOUSBFindInterfaceRequest {
    /// requested class
    pub bInterfaceClass: u16,
    /// requested subclass            
    pub bInterfaceSubClass: u16,
    /// requested protocol        
    pub bInterfaceProtocol: u16,
    /// requested alt setting
    pub bAlternateSetting: u16,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct IOCFPlugInInterface {
    // IUNKNOWN_C_GUTS
    pub _reserved: *const c_void,
    pub QueryInterface: extern "C" fn(this: *mut *mut Self, iid: REFIID, ppv: *mut LPVOID) -> HRESULT,
    pub AddRef: extern "C" fn(this: *mut *mut Self) -> ULONG,
    pub Release: extern "C" fn(this: *mut *mut Self) -> ULONG,
    // IOCFPLUGINBASE
    pub version: UInt16,
    pub revision: UInt16,
    pub Probe: extern "C" fn(
        this: *mut *mut Self,
        property_table: CFDictionaryRef,
        service: io_service_t,
        order: *mut SInt32,
    ) -> IOReturn,
    pub Start: extern "C" fn(
        this: *mut *mut Self,
        property_table: CFDictionaryRef,
        service: io_service_t,
    ) -> IOReturn,
    pub Stop: extern "C" fn(this: *mut *mut Self) -> IOReturn,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct IOUSBDeviceInterface942 {
    // IUNKNOWN_C_GUTS
    pub _reserved: *const c_void,
    pub QueryInterface: extern "C" fn(this: *mut *mut Self, iid: REFIID, ppv: *mut LPVOID) -> HRESULT,
    pub AddRef: extern "C" fn(this: *mut *mut Self) -> ULONG,
    pub Release: extern "C" fn(this: *mut *mut Self) -> ULONG,
    // IOUSBDeviceStruct942
    pub CreateDeviceAsyncEventSource:
        extern "C" fn(this: *mut *mut Self, source: *mut CFRunLoopSourceRef) -> IOReturn,
    pub GetDeviceAsyncEventSource: extern "C" fn(this: *mut *mut Self) -> CFRunLoopSourceRef,
    pub CreateDeviceAsyncPort: extern "C" fn(this: *mut *mut Self, port: *mut mach_port_t) -> IOReturn,
    pub GetDeviceAsyncPort: extern "C" fn(this: *mut *mut Self) -> mach_port_t,
    pub USBDeviceOpen: extern "C" fn(this: *mut *mut Self) -> IOReturn,
    pub USBDeviceClose: extern "C" fn(this: *mut *mut Self) -> IOReturn,
    pub GetDeviceClass: extern "C" fn(this: *mut *mut Self, devClass: *mut UInt8) -> IOReturn,
    pub GetDeviceSubClass: extern "C" fn(this: *mut *mut Self, devSubClass: *mut UInt8) -> IOReturn,
    pub GetDeviceProtocol: extern "C" fn(this: *mut *mut Self, devProtocol: *mut UInt8) -> IOReturn,
    pub GetDeviceVendor: extern "C" fn(this: *mut *mut Self, devVendor: *mut UInt16) -> IOReturn,
    pub GetDeviceProduct: extern "C" fn(this: *mut *mut Self, devProduct: *mut UInt16) -> IOReturn,
    pub GetDeviceReleaseNumber: extern "C" fn(this: *mut *mut Self, devRelNum: *mut UInt16) -> IOReturn,
    pub GetDeviceAddress: extern "C" fn(this: *mut *mut Self, addr: *mut USBDeviceAddress) -> IOReturn,
    pub GetDeviceBusPowerAvailable:
        extern "C" fn(this: *mut *mut Self, powerAvailable: *mut UInt32) -> IOReturn,
    pub GetDeviceSpeed: extern "C" fn(this: *mut *mut Self, devSpeed: *mut UInt8) -> IOReturn,
    pub GetNumberOfConfigurations:
        extern "C" fn(this: *mut *mut Self, numConfig: *mut UInt8) -> IOReturn,
    pub GetLocationID: extern "C" fn(this: *mut *mut Self, locationID: *mut UInt32) -> IOReturn,
    pub GetConfigurationDescriptorPtr: extern "C" fn(
        this: *mut *mut Self,
        configIndex: UInt8,
        desc: *mut IOUSBConfigurationDescriptorPtr,
    ) -> IOReturn,
    pub GetConfiguration: extern "C" fn(this: *mut *mut Self, configNum: *mut UInt8) -> IOReturn,
    pub SetConfiguration: extern "C" fn(this: *mut *mut Self, configNum: UInt8) -> IOReturn,
    pub GetBusFrameNumber:
        extern "C" fn(this: *mut *mut Self, frame: *mut UInt64, atTime: *mut AbsoluteTime) -> IOReturn,
    pub ResetDevice: extern "C" fn(this: *mut *mut Self) -> IOReturn,
    pub DeviceRequest: extern "C" fn(this: *mut *mut Self, req: *mut IOUSBDevRequest) -> IOReturn,
    pub DeviceRequestAsync: extern "C" fn(
        this: *mut *mut Self,
        req: *mut IOUSBDevRequest,
        callback: IOAsyncCallback1,
        refCon: *mut c_void,
    ) -> IOReturn,
    pub CreateInterfaceIterator: extern "C" fn(
        this: *mut *mut Self,
        req: *mut IOUSBFindInterfaceRequest,
        iter: *mut io_iterator_t,
    ) -> IOReturn,
    pub USBDeviceOpenSeize: extern "C" fn(this: *mut *mut Self) -> IOReturn,
    pub DeviceRequestTO: extern "C" fn(this: *mut *mut Self, req: *mut IOUSBDevRequestTO) -> IOReturn,
    pub DeviceRequestAsyncTO: extern "C" fn(
        this: *mut *mut Self,
        req: *mut IOUSBDevRequestTO,
        callback: IOAsyncCallback1,
        refCon: *mut c_void,
    ) -> IOReturn,
    pub USBDeviceSuspend: extern "C" fn(this: *mut *mut Self, suspend: Boolean) -> IOReturn,
    pub USBDeviceAbortPipeZero: extern "C" fn(this: *mut *mut Self) -> IOReturn,
    pub USBGetManufacturerStringIndex: extern "C" fn(this: *mut *mut Self, msi: *mut UInt8) -> IOReturn,
    pub USBGetProductStringIndex: extern "C" fn(this: *mut *mut Self, psi: *mut UInt8) -> IOReturn,
    pub USBGetSerialNumberStringIndex: extern "C" fn(this: *mut *mut Self, snsi: *mut UInt8) -> IOReturn,
    pub USBDeviceReEnumerate: extern "C" fn(this: *mut *mut Self, options: UInt32) -> IOReturn,
    pub GetBusMicroFrameNumber: extern "C" fn(
        this: *mut *mut Self,
        microFrame: *mut UInt64,
        atTime: *mut AbsoluteTime,
    ) -> IOReturn,
    pub GetIOUSBLibVersion: extern "C" fn(
        this: *mut *mut Self,
        ioUSBLibVersion: *mut NumVersion,
        usbFamilyVersion: *mut NumVersion,
    ) -> IOReturn,
    pub GetBusFrameNumberWithTime:
        extern "C" fn(this: *mut *mut Self, frame: *mut UInt64, atTime: *mut AbsoluteTime) -> IOReturn,
    pub GetUSBDeviceInformation: extern "C" fn(this: *mut *mut Self, info: *mut UInt32) -> IOReturn,
    pub RequestExtraPower: extern "C" fn(
        this: *mut *mut Self,
        type_: UInt32,
        requestedPower: UInt32,
        powerAvailable: *mut UInt32,
    ) -> IOReturn,
    pub ReturnExtraPower:
        extern "C" fn(this: *mut *mut Self, type_: UInt32, powerReturned: UInt32) -> IOReturn,
    pub GetExtraPowerAllocated:
        extern "C" fn(this: *mut *mut Self, type_: UInt32, powerAllocated: *mut UInt32) -> IOReturn,
}
