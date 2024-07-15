#![no_std]
#![feature(c_variadic)]
mod ehci;

pub use cherryusb_rs_sys;
use core::ffi::c_char;
use core::ffi::{c_int, c_long, c_uint};
use core::sync::atomic::AtomicBool;
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};

pub fn get_var(id: u8) -> u32 {
    unsafe { cherryusb_rs_sys::strlen(core::ptr::null()) as u32 }
}

pub struct Hid {
    is_busy: bool,
}

static IS_BUSY: AtomicBool = AtomicBool::new(false);

unsafe extern "C" fn printf(fmt: *const c_char, ...) -> c_int {
    0
}

unsafe extern "C" fn usbd_get_dwc2_gccfg_conf(reg_base: c_uint) -> c_uint {
    unsafe {
        let mut gotgctl = reg_base as *mut u32;
        // USB_OTG_GOTGCTL_BVALOEN
        *gotgctl |= (1 << 6);
        // USB_OTG_GOTGCTL_BVALOVAL
        *gotgctl |= (1 << 7);
    }

    1 << 16
}

extern "C" fn usbd_hid_int_callback(busid: u8, ep: u8, nbytes: u32) {
    IS_BUSY.store(false, core::sync::atomic::Ordering::SeqCst);
}

extern "C" fn usbd_event_hander(busid: u8, event: u8) {
    if event == 7 {
        // USBD_EVENT_CONFIGURED
        IS_BUSY.store(false, core::sync::atomic::Ordering::SeqCst);
    }
}

pub fn keyboard_init(busid: u8, reg_base: u32) {
    let descriptor = usbd_hid::descriptor::KeyboardReport::desc();
    let mut interface0 = cherryusb_rs_sys::usbd_interface::default();
    let mut endpoint = cherryusb_rs_sys::usbd_endpoint::default();
    endpoint.ep_addr = 0x81;
    endpoint.ep_cb = Some(usbd_hid_int_callback);
    unsafe {
        let interface = cherryusb_rs_sys::usbd_hid_init_intf(
            busid,
            &mut interface0,
            KeyboardReport::desc().as_ptr(),
            KeyboardReport::desc().len() as u32,
        );
        cherryusb_rs_sys::usbd_desc_register(busid, descriptor.as_ptr());
        cherryusb_rs_sys::usbd_add_interface(busid, interface);
        cherryusb_rs_sys::usbd_add_endpoint(busid, &mut endpoint);
        cherryusb_rs_sys::usbd_initialize(busid, reg_base, Some(usbd_event_hander));
    };
}

pub fn keyboard_test(busid: u8) -> i32 {
    let send_buffer: [u8; 8] = [0, 0, 4, 0, 0, 0, 0, 0];

    let res = unsafe { cherryusb_rs_sys::usbd_ep_start_write(0, 0x81, send_buffer.as_ptr(), 8) };

    res
}
