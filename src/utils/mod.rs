//! Utilities for use within wlroots-rs that are not directly related to Wayland
//! or compositors.

pub mod edges;
pub mod log;
pub mod region;

// Rust specific utilities that don't wrap a wlroots utility.
mod handle;
mod string;
mod time;

pub use self::handle::*;
pub(crate) use self::string::{c_to_rust_string, safe_as_cstring};
pub use self::time::{current_time, ToMs};

/// Handle unwinding from a panic, used in conjunction with
/// `::std::panic::catch_unwind`.
///
/// When a panic occurs, we terminate the compositor and let the rest
/// of the code run.
#[cfg(feature = "unstable")]
pub(crate) unsafe fn handle_unwind<T>(res: ::std::thread::Result<T>) {
    match res {
        Ok(_) => {},
        Err(err) => {
            if crate::compositor::COMPOSITOR_PTR == 0 as *mut _ {
                ::std::process::abort();
            }
            (&mut *crate::compositor::COMPOSITOR_PTR).save_panic_error(err);
            crate::compositor::terminate()
        }
    }
}
