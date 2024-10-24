// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use glib::{translate::*};

glib::wrapper! {
    #[doc(alias = "PhoshScreenshotManager")]
    pub struct ScreenshotManager(Object<ffi::PhoshScreenshotManager, ffi::PhoshScreenshotManagerClass>);

    match fn {
        type_ => || ffi::phosh_screenshot_manager_get_type(),
    }
}

impl ScreenshotManager {
    #[doc(alias = "phosh_screenshot_manager_new")]
    pub fn new() -> ScreenshotManager {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::phosh_screenshot_manager_new())
        }
    }

    #[doc(alias = "phosh_screenshot_manager_do_screenshot")]
    pub fn do_screenshot(&self, area: Option<&gdk::Rectangle>, filename: Option<&str>, include_cursor: bool) -> bool {
        unsafe {
            from_glib(ffi::phosh_screenshot_manager_do_screenshot(self.to_glib_none().0, area.to_glib_none().0, filename.to_glib_none().0, include_cursor.into_glib()))
        }
    }
}

impl Default for ScreenshotManager {
                     fn default() -> Self {
                         Self::new()
                     }
                 }
