//! TODO Documentation

use std::{cell::Cell, ptr::NonNull, rc::Rc};

use wlroots_sys::{wlr_input_device, wlr_touch};

pub use crate::events::touch_events as event;
pub use crate::manager::touch_handler::*;
use crate::{
    input::{self, InputState},
    utils::{self, HandleErr, HandleResult, Handleable}
};

pub type Handle = utils::Handle<NonNull<wlr_input_device>, wlr_touch, Touch>;

#[derive(Debug)]
pub struct Touch {
    /// The structure that ensures weak handles to this structure are still
    /// alive.
    ///
    /// They contain weak handles, and will safely not use dead memory when this
    /// is freed by wlroots.
    ///
    /// If this is `None`, then this is from an upgraded `touch::Handle`, and
    /// the operations are **unchecked**.
    /// This is means safe operations might fail, but only if you use the unsafe
    /// marked function `upgrade` on a `touch::Handle`.
    liveliness: Rc<Cell<bool>>,
    /// The device that refers to this touch.
    device: input::Device,
    /// The underlying touch data.
    touch: NonNull<wlr_touch>
}

impl Touch {
    /// Tries to convert an input device to a Touch.
    ///
    /// Returns none if it is of a different input variant.
    ///
    /// # Safety
    /// This creates a totally new Touch (e.g with its own reference count)
    /// so only do this once per `wlr_input_device`!
    pub(crate) unsafe fn new_from_input_device(device: *mut wlr_input_device) -> Option<Self> {
        use wlroots_sys::wlr_input_device_type::*;
        match (*device).type_ {
            WLR_INPUT_DEVICE_TOUCH => {
                let touch = NonNull::new((*device).__bindgen_anon_1.touch).expect("Touch pointer was null");
                let liveliness = Rc::new(Cell::new(false));
                let handle = Rc::downgrade(&liveliness);
                let state = Box::new(InputState {
                    handle,
                    device: input::Device::from_ptr(device)
                });
                (*touch.as_ptr()).data = Box::into_raw(state) as *mut _;
                Some(Touch {
                    liveliness,
                    device: input::Device::from_ptr(device),
                    touch
                })
            },
            _ => None
        }
    }

    /// Gets the wlr_input_device associated with this `Touch`.
    pub fn input_device(&self) -> &input::Device {
        &self.device
    }
}
impl Drop for Touch {
    fn drop(&mut self) {
        if Rc::strong_count(&self.liveliness) == 1 {
            wlr_log!(WLR_DEBUG, "Dropped Touch {:p}", self.touch.as_ptr());
            unsafe {
                let _ = Box::from_raw((*self.touch.as_ptr()).data as *mut input::Device);
            }
            let weak_count = Rc::weak_count(&self.liveliness);
            if weak_count > 0 {
                wlr_log!(
                    WLR_DEBUG,
                    "Still {} weak pointers to Touch {:p}",
                    weak_count,
                    self.touch.as_ptr()
                );
            }
        }
    }
}

impl Handleable<NonNull<wlr_input_device>, wlr_touch> for Touch {
    #[doc(hidden)]
    unsafe fn from_ptr(touch: *mut wlr_touch) -> Option<Self> {
        let touch = NonNull::new(touch)?;
        let data = Box::from_raw((*touch.as_ptr()).data as *mut InputState);
        let handle = data.handle.clone();
        let device = data.device.clone();
        (*touch.as_ptr()).data = Box::into_raw(data) as *mut _;
        Some(Touch {
            liveliness: handle.upgrade().unwrap(),
            device,
            touch
        })
    }

    #[doc(hidden)]
    unsafe fn as_ptr(&self) -> *mut wlr_touch {
        self.touch.as_ptr()
    }

    #[doc(hidden)]
    unsafe fn from_handle(handle: &Handle) -> HandleResult<Self> {
        let liveliness = handle.handle.upgrade().ok_or(HandleErr::AlreadyDropped)?;
        let device = handle.data.ok_or(HandleErr::AlreadyDropped)?;
        Ok(Touch {
            liveliness,
            // NOTE Rationale for cloning:
            // If we already dropped we don't reach this point.
            device: input::Device { device },
            touch: handle.as_non_null()
        })
    }

    fn weak_reference(&self) -> Handle {
        Handle {
            ptr: self.touch,
            handle: Rc::downgrade(&self.liveliness),
            // NOTE Rationale for cloning:
            // Since we have a strong reference already,
            // the input must still be alive.
            data: unsafe { Some(self.device.as_non_null()) },
            _marker: std::marker::PhantomData
        }
    }
}
