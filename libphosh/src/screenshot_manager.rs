// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use std::ptr;

use crate::{prelude::*, ScreenshotManager};

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::ScreenshotManager>> Sealed for T {}
}

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of
/// [`ScreenshotManager`](crate::ScreenshotManger).
pub trait ScreenshotManagerExtManual: sealed::Sealed + IsA<ScreenshotManager> + 'static {
    //#[doc(alias = "phosh_screenshot_manager_do_screenshot")]
    fn do_screenshot(&self, filename: &str, include_cursor: bool) -> bool {
        unsafe {
            from_glib(ffi::phosh_screenshot_manager_do_screenshot(self.as_ref().to_glib_none().0,
                                                                  ptr::null(),
                                                                  filename.to_glib_none().0,
                                                                  include_cursor.into_glib()))
        }
    }
}

impl<O: IsA<ScreenshotManager>> ScreenshotManagerExtManual for O {}
