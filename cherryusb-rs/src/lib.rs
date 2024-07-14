#![no_std]
mod ehci;

pub use cherryusb_rs_sys;

pub fn get_var() -> u32 {
    unsafe { cherryusb_rs_sys::mrand48() as u32 }
}
