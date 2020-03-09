mod ffi;
use ffi::*;
use core::mem::MaybeUninit;
use mach::kern_return::kern_return_t;
use core::fmt;
use std::ffi::CString;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum Error {
    MatchServiceFailed,
    IoIteratorInvalid,
    Utf8(Utf8Error),
    Kernel(kern_return_t)
}

pub type Result<T> = core::result::Result<T, Error>;

pub fn devices() -> Result<Devices> {
    Devices::new_usb_all()
}

#[derive(Debug)]
pub struct Devices {
    iter: io_iterator_t,
}

impl Devices {
    fn new_usb_all() -> Result<Self> {
        let matching_dict = unsafe { IOServiceMatching(kIOUSBHostDeviceClassName()) };
        if matching_dict == core::ptr::null_mut() {
            return Err(Error::MatchServiceFailed);
        }
        let mut iter: MaybeUninit<io_iterator_t> = MaybeUninit::uninit();
        let kr = unsafe { IOServiceGetMatchingServices(
            kIOMasterPortDefault, 
            matching_dict, 
            iter.as_mut_ptr()
        ) };
        if kr != mach::kern_return::KERN_SUCCESS {
            return Err(Error::Kernel(kr));
        }
        Ok(Devices {
            iter: unsafe { iter.assume_init() }
        } )
    }
}

impl Drop for Devices {
    fn drop(&mut self) {
        unsafe { IOObjectRelease(self.iter) };
    } 
}

impl Iterator for Devices {
    type Item = Result<Device>;

    fn next(&mut self) -> Option<Self::Item> {
        let is_valid = unsafe { IOIteratorIsValid(self.iter) };
        if !is_valid {
            unsafe { IOIteratorReset(self.iter) };
            return Some(Err(Error::IoIteratorInvalid))
        }
        let service = unsafe { IOIteratorNext(self.iter) };
        if service == 0 {
            return None;
        }
        Some(Ok(Device::from_service(service)))
    }
}

pub struct Device {
    service: io_service_t,
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { IOObjectRelease(self.service) };
    }
}

impl Device {
    fn from_service(service: io_service_t) -> Self {
        Self { service }
    }

    fn read_name_string(&self) -> Result<String> {
        let mut dst = Box::new([0i8; 128]);
        let dst_ptr = dst.as_mut_ptr();
        let kr = unsafe { IORegistryEntryGetName(self.service, dst_ptr) };
        if kr != mach::kern_return::KERN_SUCCESS {
            return Err(Error::Kernel(kr))
        }
        let c_string = unsafe { CString::from_raw(Box::into_raw(dst) as *mut i8) };
        c_string.into_string().map_err(|e| Error::Utf8(e.utf8_error()))
    }
}

impl fmt::Debug for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.read_name_string() {
            Ok(s) => write!(f, "{:?}", s),
            Err(e) => write!(f, "{:?}", e),
        }
    }
}

fn main() -> Result<()> {
    for device in dbg!(devices()?) {
        dbg!(device?);
    }
    Ok(())
}
