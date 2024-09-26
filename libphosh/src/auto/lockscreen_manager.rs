// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{Lockscreen,LockscreenPage};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "PhoshLockscreenManager")]
    pub struct LockscreenManager(Object<ffi::PhoshLockscreenManager, ffi::PhoshLockscreenManagerClass>);

    match fn {
        type_ => || ffi::phosh_lockscreen_manager_get_type(),
    }
}

impl LockscreenManager {
    //#[doc(alias = "phosh_lockscreen_manager_new")]
    //pub fn new(calls_manager: /*Ignored*/&CallsManager) -> LockscreenManager {
    //    unsafe { TODO: call ffi:phosh_lockscreen_manager_new() }
    //}

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`LockscreenManager`] objects.
            ///
            /// This method returns an instance of [`LockscreenManagerBuilder`](crate::builders::LockscreenManagerBuilder) which can be used to create [`LockscreenManager`] objects.
            pub fn builder() -> LockscreenManagerBuilder {
                LockscreenManagerBuilder::new()
            }
        

    #[doc(alias = "phosh_lockscreen_manager_get_active_time")]
    #[doc(alias = "get_active_time")]
    pub fn active_time(&self) -> i64 {
        unsafe {
            ffi::phosh_lockscreen_manager_get_active_time(self.to_glib_none().0)
        }
    }

    #[doc(alias = "phosh_lockscreen_manager_get_locked")]
    #[doc(alias = "get_locked")]
    pub fn is_locked(&self) -> bool {
        unsafe {
            from_glib(ffi::phosh_lockscreen_manager_get_locked(self.to_glib_none().0))
        }
    }

    #[doc(alias = "phosh_lockscreen_manager_get_lockscreen")]
    #[doc(alias = "get_lockscreen")]
    pub fn lockscreen(&self) -> Option<Lockscreen> {
        unsafe {
            from_glib_none(ffi::phosh_lockscreen_manager_get_lockscreen(self.to_glib_none().0))
        }
    }

    #[doc(alias = "phosh_lockscreen_manager_get_page")]
    #[doc(alias = "get_page")]
    pub fn page(&self) -> LockscreenPage {
        unsafe {
            from_glib(ffi::phosh_lockscreen_manager_get_page(self.to_glib_none().0))
        }
    }

    #[doc(alias = "phosh_lockscreen_manager_get_timeout")]
    #[doc(alias = "get_timeout")]
    pub fn timeout(&self) -> i32 {
        unsafe {
            ffi::phosh_lockscreen_manager_get_timeout(self.to_glib_none().0)
        }
    }

    #[doc(alias = "phosh_lockscreen_manager_set_locked")]
    pub fn set_locked(&self, state: bool) {
        unsafe {
            ffi::phosh_lockscreen_manager_set_locked(self.to_glib_none().0, state.into_glib());
        }
    }

    #[doc(alias = "phosh_lockscreen_manager_set_page")]
    pub fn set_page(&self, page: LockscreenPage) -> bool {
        unsafe {
            from_glib(ffi::phosh_lockscreen_manager_set_page(self.to_glib_none().0, page.into_glib()))
        }
    }

    #[doc(alias = "phosh_lockscreen_manager_set_timeout")]
    pub fn set_timeout(&self, timeout: i32) {
        unsafe {
            ffi::phosh_lockscreen_manager_set_timeout(self.to_glib_none().0, timeout);
        }
    }

    //#[doc(alias = "calls-manager")]
    //pub fn calls_manager(&self) -> /*Ignored*/Option<CallsManager> {
    //    ObjectExt::property(self, "calls-manager")
    //}

    #[doc(alias = "wakeup-outputs")]
    pub fn connect_wakeup_outputs<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn wakeup_outputs_trampoline<F: Fn(&LockscreenManager) + 'static>(this: *mut ffi::PhoshLockscreenManager, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"wakeup-outputs\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(wakeup_outputs_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "locked")]
    pub fn connect_locked_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_locked_trampoline<F: Fn(&LockscreenManager) + 'static>(this: *mut ffi::PhoshLockscreenManager, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::locked\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_locked_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl Default for LockscreenManager {
                     fn default() -> Self {
                         glib::object::Object::new::<Self>()
                     }
                 }

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`LockscreenManager`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct LockscreenManagerBuilder {
            builder: glib::object::ObjectBuilder<'static, LockscreenManager>,
        }

        impl LockscreenManagerBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            //pub fn calls_manager(self, calls_manager: /*Ignored*/&CallsManager) -> Self {
                        //    Self { builder: self.builder.property("calls-manager", calls_manager), }
                        //}

                            pub fn locked(self, locked: bool) -> Self {
                            Self { builder: self.builder.property("locked", locked), }
                        }

    // rustdoc-stripper-ignore-next
    /// Build the [`LockscreenManager`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> LockscreenManager {
    self.builder.build() }
}