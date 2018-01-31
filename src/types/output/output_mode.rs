//! TODO Documentation

use wlroots_sys::wlr_output_mode;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OutputMode {
    output_mode: *mut wlr_output_mode
}

impl OutputMode {
    pub(crate) unsafe fn new(output_mode: *mut wlr_output_mode) -> Self {
        OutputMode { output_mode }
    }

    pub(crate) unsafe fn as_ptr(&self) -> *mut wlr_output_mode {
        self.output_mode
    }

    /// Gets the flags set on this OutputMode.
    pub fn flags(&self) -> u32 {
        unsafe { (*self.output_mode).flags }
    }

    /// Gets the dimensions of this OutputMode.
    ///
    /// Returned value is (width, height)
    pub fn dimensions(&self) -> (i32, i32) {
        unsafe { ((*self.output_mode).width, (*self.output_mode).height) }
    }

    /// Get the refresh value of the output.
    pub fn refresh(&self) -> i32 {
        unsafe { (*self.output_mode).refresh }
    }
}