// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "PhoshShell")]
    pub struct Shell(Object<ffi::PhoshShell, ffi::PhoshShellClass>);

    match fn {
        type_ => || ffi::phosh_shell_get_type(),
    }
}

impl Shell {
        pub const NONE: Option<&'static Shell> = None;
    

    #[doc(alias = "phosh_shell_new")]
    pub fn new() -> Shell {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::phosh_shell_new())
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`Shell`] objects.
            ///
            /// This method returns an instance of [`ShellBuilder`](crate::builders::ShellBuilder) which can be used to create [`Shell`] objects.
            pub fn builder() -> ShellBuilder {
                ShellBuilder::new()
            }
        

    //#[doc(alias = "phosh_shell_get_debug_flags")]
    //#[doc(alias = "get_debug_flags")]
    //pub fn debug_flags() -> /*Ignored*/ShellDebugFlags {
    //    unsafe { TODO: call ffi:phosh_shell_get_debug_flags() }
    //}

    #[doc(alias = "phosh_shell_get_default")]
    #[doc(alias = "get_default")]
    #[allow(clippy::should_implement_trait)]    pub fn default() -> Shell {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::phosh_shell_get_default())
        }
    }
}

impl Default for Shell {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`Shell`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ShellBuilder {
            builder: glib::object::ObjectBuilder<'static, Shell>,
        }

        impl ShellBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn docked(self, docked: bool) -> Self {
                            Self { builder: self.builder.property("docked", docked), }
                        }

                            pub fn locked(self, locked: bool) -> Self {
                            Self { builder: self.builder.property("locked", locked), }
                        }

                            //pub fn primary_monitor(self, primary_monitor: &impl IsA</*Ignored*/Monitor>) -> Self {
                        //    Self { builder: self.builder.property("primary-monitor", primary_monitor.clone().upcast()), }
                        //}

    // rustdoc-stripper-ignore-next
    /// Build the [`Shell`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Shell {
    self.builder.build() }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Shell>> Sealed for T {}
}

pub trait ShellExt: IsA<Shell> + sealed::Sealed + 'static {
    //#[doc(alias = "phosh_shell_activate_action")]
    //fn activate_action(&self, action: &str, parameter: /*Ignored*/&glib::Variant) -> bool {
    //    unsafe { TODO: call ffi:phosh_shell_activate_action() }
    //}

    //#[doc(alias = "phosh_shell_add_global_keyboard_action_entries")]
    //fn add_global_keyboard_action_entries(&self, actions: /*Ignored*/&gio::ActionEntry, n_entries: i32, user_data: /*Unimplemented*/Option<Basic: Pointer>) {
    //    unsafe { TODO: call ffi:phosh_shell_add_global_keyboard_action_entries() }
    //}

    #[doc(alias = "phosh_shell_enable_power_save")]
    fn enable_power_save(&self, enable: bool) {
        unsafe {
            ffi::phosh_shell_enable_power_save(self.as_ref().to_glib_none().0, enable.into_glib());
        }
    }

    #[doc(alias = "phosh_shell_fade_out")]
    fn fade_out(&self, timeout: u32) {
        unsafe {
            ffi::phosh_shell_fade_out(self.as_ref().to_glib_none().0, timeout);
        }
    }

    //#[doc(alias = "phosh_shell_get_app_launch_context")]
    //#[doc(alias = "get_app_launch_context")]
    //fn app_launch_context(&self) -> /*Ignored*/gdk::AppLaunchContext {
    //    unsafe { TODO: call ffi:phosh_shell_get_app_launch_context() }
    //}

    //#[doc(alias = "phosh_shell_get_app_tracker")]
    //#[doc(alias = "get_app_tracker")]
    //fn app_tracker(&self) -> /*Ignored*/AppTracker {
    //    unsafe { TODO: call ffi:phosh_shell_get_app_tracker() }
    //}

    //#[doc(alias = "phosh_shell_get_background_manager")]
    //#[doc(alias = "get_background_manager")]
    //fn background_manager(&self) -> /*Ignored*/BackgroundManager {
    //    unsafe { TODO: call ffi:phosh_shell_get_background_manager() }
    //}

    #[doc(alias = "phosh_shell_get_blanked")]
    #[doc(alias = "get_blanked")]
    fn is_blanked(&self) -> bool {
        unsafe {
            from_glib(ffi::phosh_shell_get_blanked(self.as_ref().to_glib_none().0))
        }
    }

    //#[doc(alias = "phosh_shell_get_builtin_monitor")]
    //#[doc(alias = "get_builtin_monitor")]
    //fn builtin_monitor(&self) -> /*Ignored*/Option<Monitor> {
    //    unsafe { TODO: call ffi:phosh_shell_get_builtin_monitor() }
    //}

    //#[doc(alias = "phosh_shell_get_calls_manager")]
    //#[doc(alias = "get_calls_manager")]
    //fn calls_manager(&self) -> /*Ignored*/CallsManager {
    //    unsafe { TODO: call ffi:phosh_shell_get_calls_manager() }
    //}

    #[doc(alias = "phosh_shell_get_docked")]
    #[doc(alias = "get_docked")]
    fn is_docked(&self) -> bool {
        unsafe {
            from_glib(ffi::phosh_shell_get_docked(self.as_ref().to_glib_none().0))
        }
    }

    //#[doc(alias = "phosh_shell_get_docked_manager")]
    //#[doc(alias = "get_docked_manager")]
    //fn docked_manager(&self) -> /*Ignored*/DockedManager {
    //    unsafe { TODO: call ffi:phosh_shell_get_docked_manager() }
    //}

    //#[doc(alias = "phosh_shell_get_emergency_calls_manager")]
    //#[doc(alias = "get_emergency_calls_manager")]
    //fn emergency_calls_manager(&self) -> /*Ignored*/EmergencyCallsManager {
    //    unsafe { TODO: call ffi:phosh_shell_get_emergency_calls_manager() }
    //}

    //#[doc(alias = "phosh_shell_get_feedback_manager")]
    //#[doc(alias = "get_feedback_manager")]
    //fn feedback_manager(&self) -> /*Ignored*/FeedbackManager {
    //    unsafe { TODO: call ffi:phosh_shell_get_feedback_manager() }
    //}

    //#[doc(alias = "phosh_shell_get_gtk_mount_manager")]
    //#[doc(alias = "get_gtk_mount_manager")]
    //fn gtk_mount_manager(&self) -> /*Ignored*/GtkMountManager {
    //    unsafe { TODO: call ffi:phosh_shell_get_gtk_mount_manager() }
    //}

    //#[doc(alias = "phosh_shell_get_hks_manager")]
    //#[doc(alias = "get_hks_manager")]
    //fn hks_manager(&self) -> /*Ignored*/HksManager {
    //    unsafe { TODO: call ffi:phosh_shell_get_hks_manager() }
    //}

    //#[doc(alias = "phosh_shell_get_launcher_entry_manager")]
    //#[doc(alias = "get_launcher_entry_manager")]
    //fn launcher_entry_manager(&self) -> /*Ignored*/LauncherEntryManager {
    //    unsafe { TODO: call ffi:phosh_shell_get_launcher_entry_manager() }
    //}

    //#[doc(alias = "phosh_shell_get_layout_manager")]
    //#[doc(alias = "get_layout_manager")]
    //fn layout_manager(&self) -> /*Ignored*/LayoutManager {
    //    unsafe { TODO: call ffi:phosh_shell_get_layout_manager() }
    //}

    //#[doc(alias = "phosh_shell_get_location_manager")]
    //#[doc(alias = "get_location_manager")]
    //fn location_manager(&self) -> /*Ignored*/LocationManager {
    //    unsafe { TODO: call ffi:phosh_shell_get_location_manager() }
    //}

    #[doc(alias = "phosh_shell_get_locked")]
    #[doc(alias = "get_locked")]
    fn is_locked(&self) -> bool {
        unsafe {
            from_glib(ffi::phosh_shell_get_locked(self.as_ref().to_glib_none().0))
        }
    }

    //#[doc(alias = "phosh_shell_get_lockscreen_manager")]
    //#[doc(alias = "get_lockscreen_manager")]
    //fn lockscreen_manager(&self) -> /*Ignored*/LockscreenManager {
    //    unsafe { TODO: call ffi:phosh_shell_get_lockscreen_manager() }
    //}

    #[doc(alias = "phosh_shell_get_lockscreen_type")]
    #[doc(alias = "get_lockscreen_type")]
    fn lockscreen_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::phosh_shell_get_lockscreen_type(self.as_ref().to_glib_none().0))
        }
    }

    //#[doc(alias = "phosh_shell_get_mode_manager")]
    //#[doc(alias = "get_mode_manager")]
    //fn mode_manager(&self) -> /*Ignored*/ModeManager {
    //    unsafe { TODO: call ffi:phosh_shell_get_mode_manager() }
    //}

    //#[doc(alias = "phosh_shell_get_monitor_manager")]
    //#[doc(alias = "get_monitor_manager")]
    //fn monitor_manager(&self) -> /*Ignored*/MonitorManager {
    //    unsafe { TODO: call ffi:phosh_shell_get_monitor_manager() }
    //}

    //#[doc(alias = "phosh_shell_get_osk_manager")]
    //#[doc(alias = "get_osk_manager")]
    //fn osk_manager(&self) -> /*Ignored*/OskManager {
    //    unsafe { TODO: call ffi:phosh_shell_get_osk_manager() }
    //}

    //#[doc(alias = "phosh_shell_get_primary_monitor")]
    //#[doc(alias = "get_primary_monitor")]
    //fn primary_monitor(&self) -> /*Ignored*/Option<Monitor> {
    //    unsafe { TODO: call ffi:phosh_shell_get_primary_monitor() }
    //}

    //#[doc(alias = "phosh_shell_get_rotation_manager")]
    //#[doc(alias = "get_rotation_manager")]
    //fn rotation_manager(&self) -> /*Ignored*/RotationManager {
    //    unsafe { TODO: call ffi:phosh_shell_get_rotation_manager() }
    //}

    //#[doc(alias = "phosh_shell_get_screen_saver_manager")]
    //#[doc(alias = "get_screen_saver_manager")]
    //fn screen_saver_manager(&self) -> /*Ignored*/ScreenSaverManager {
    //    unsafe { TODO: call ffi:phosh_shell_get_screen_saver_manager() }
    //}

    //#[doc(alias = "phosh_shell_get_screenshot_manager")]
    //#[doc(alias = "get_screenshot_manager")]
    //fn screenshot_manager(&self) -> /*Ignored*/ScreenshotManager {
    //    unsafe { TODO: call ffi:phosh_shell_get_screenshot_manager() }
    //}

    //#[doc(alias = "phosh_shell_get_session_manager")]
    //#[doc(alias = "get_session_manager")]
    //fn session_manager(&self) -> /*Ignored*/SessionManager {
    //    unsafe { TODO: call ffi:phosh_shell_get_session_manager() }
    //}

    #[doc(alias = "phosh_shell_get_show_splash")]
    #[doc(alias = "get_show_splash")]
    fn shows_splash(&self) -> bool {
        unsafe {
            from_glib(ffi::phosh_shell_get_show_splash(self.as_ref().to_glib_none().0))
        }
    }

    //#[doc(alias = "phosh_shell_get_state")]
    //#[doc(alias = "get_state")]
    //fn state(&self) -> /*Ignored*/ShellStateFlags {
    //    unsafe { TODO: call ffi:phosh_shell_get_state() }
    //}

    //#[doc(alias = "phosh_shell_get_toplevel_manager")]
    //#[doc(alias = "get_toplevel_manager")]
    //fn toplevel_manager(&self) -> /*Ignored*/ToplevelManager {
    //    unsafe { TODO: call ffi:phosh_shell_get_toplevel_manager() }
    //}

    //#[doc(alias = "phosh_shell_get_torch_manager")]
    //#[doc(alias = "get_torch_manager")]
    //fn torch_manager(&self) -> /*Ignored*/TorchManager {
    //    unsafe { TODO: call ffi:phosh_shell_get_torch_manager() }
    //}

    #[doc(alias = "phosh_shell_get_usable_area")]
    #[doc(alias = "get_usable_area")]
    fn usable_area(&self) -> (i32, i32, i32, i32) {
        unsafe {
            let mut x = std::mem::MaybeUninit::uninit();
            let mut y = std::mem::MaybeUninit::uninit();
            let mut width = std::mem::MaybeUninit::uninit();
            let mut height = std::mem::MaybeUninit::uninit();
            ffi::phosh_shell_get_usable_area(self.as_ref().to_glib_none().0, x.as_mut_ptr(), y.as_mut_ptr(), width.as_mut_ptr(), height.as_mut_ptr());
            (x.assume_init(), y.assume_init(), width.assume_init(), height.assume_init())
        }
    }

    //#[doc(alias = "phosh_shell_get_vpn_manager")]
    //#[doc(alias = "get_vpn_manager")]
    //fn vpn_manager(&self) -> /*Ignored*/VpnManager {
    //    unsafe { TODO: call ffi:phosh_shell_get_vpn_manager() }
    //}

    //#[doc(alias = "phosh_shell_get_wifi_manager")]
    //#[doc(alias = "get_wifi_manager")]
    //fn wifi_manager(&self) -> /*Ignored*/WifiManager {
    //    unsafe { TODO: call ffi:phosh_shell_get_wifi_manager() }
    //}

    //#[doc(alias = "phosh_shell_get_wwan")]
    //#[doc(alias = "get_wwan")]
    //fn wwan(&self) -> /*Ignored*/WWan {
    //    unsafe { TODO: call ffi:phosh_shell_get_wwan() }
    //}

    #[doc(alias = "phosh_shell_is_session_active")]
    fn is_session_active(&self) -> bool {
        unsafe {
            from_glib(ffi::phosh_shell_is_session_active(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "phosh_shell_is_startup_finished")]
    fn is_startup_finished(&self) -> bool {
        unsafe {
            from_glib(ffi::phosh_shell_is_startup_finished(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "phosh_shell_lock")]
    fn lock(&self) {
        unsafe {
            ffi::phosh_shell_lock(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "phosh_shell_set_default")]
    fn set_default(&self) {
        unsafe {
            ffi::phosh_shell_set_default(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "phosh_shell_set_locked")]
    fn set_locked(&self, locked: bool) {
        unsafe {
            ffi::phosh_shell_set_locked(self.as_ref().to_glib_none().0, locked.into_glib());
        }
    }

    //#[doc(alias = "phosh_shell_set_primary_monitor")]
    //fn set_primary_monitor(&self, monitor: /*Ignored*/&Monitor) {
    //    unsafe { TODO: call ffi:phosh_shell_set_primary_monitor() }
    //}

    //#[doc(alias = "phosh_shell_set_state")]
    //fn set_state(&self, state: /*Ignored*/ShellStateFlags, enabled: bool) {
    //    unsafe { TODO: call ffi:phosh_shell_set_state() }
    //}

    #[doc(alias = "phosh_shell_started_by_display_manager")]
    fn started_by_display_manager(&self) -> bool {
        unsafe {
            from_glib(ffi::phosh_shell_started_by_display_manager(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "phosh_shell_unlock")]
    fn unlock(&self) {
        unsafe {
            ffi::phosh_shell_unlock(self.as_ref().to_glib_none().0);
        }
    }

    fn set_docked(&self, docked: bool) {
        ObjectExt::set_property(self.as_ref(),"docked", docked)
    }

    //#[doc(alias = "shell-state")]
    //fn shell_state(&self) -> /*Ignored*/ShellStateFlags {
    //    ObjectExt::property(self.as_ref(), "shell-state")
    //}

    #[doc(alias = "ready")]
    fn connect_ready<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn ready_trampoline<P: IsA<Shell>, F: Fn(&P) + 'static>(this: *mut ffi::PhoshShell, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Shell::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"ready\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(ready_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "builtin-monitor")]
    fn connect_builtin_monitor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_builtin_monitor_trampoline<P: IsA<Shell>, F: Fn(&P) + 'static>(this: *mut ffi::PhoshShell, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Shell::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::builtin-monitor\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_builtin_monitor_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "docked")]
    fn connect_docked_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_docked_trampoline<P: IsA<Shell>, F: Fn(&P) + 'static>(this: *mut ffi::PhoshShell, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Shell::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::docked\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_docked_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "locked")]
    fn connect_locked_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_locked_trampoline<P: IsA<Shell>, F: Fn(&P) + 'static>(this: *mut ffi::PhoshShell, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Shell::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::locked\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_locked_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "primary-monitor")]
    fn connect_primary_monitor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_primary_monitor_trampoline<P: IsA<Shell>, F: Fn(&P) + 'static>(this: *mut ffi::PhoshShell, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Shell::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::primary-monitor\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_primary_monitor_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "shell-state")]
    fn connect_shell_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_shell_state_trampoline<P: IsA<Shell>, F: Fn(&P) + 'static>(this: *mut ffi::PhoshShell, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Shell::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::shell-state\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_shell_state_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<Shell>> ShellExt for O {}
