use wlroots_sys::{
    libinput_device, wl_display, wlr_backend, wlr_input_device_is_libinput, wlr_libinput_backend_create,
    wlr_libinput_get_device_handle
};

use {backend::Session, input};

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Libinput {
    pub(crate) backend: *mut wlr_backend
}

impl Libinput {
    pub unsafe fn new(display: *mut wl_display, session: Session) -> Self {
        let backend = wlr_libinput_backend_create(display, session.as_ptr());
        if backend.is_null() {
            panic!("Could not construct Wayland backend");
        }
        Libinput { backend }
    }

    /// Get the underlying libinput_device handle for the given input device.
    pub unsafe fn device_handle(input_device: &input::Device) -> *mut libinput_device {
        wlr_libinput_get_device_handle(input_device.as_ptr())
    }

    pub fn is_libinput_input_device(&self, input_device: &input::Device) -> bool {
        unsafe { wlr_input_device_is_libinput(input_device.as_ptr()) }
    }
}
