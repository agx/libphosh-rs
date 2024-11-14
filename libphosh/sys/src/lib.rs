// Generated by gir (https://github.com/gtk-rs/gir @ 5223ce91b97a)
// from ../.. (@ eb5ea3018a51+)
// from ../../gir-files (@ 6cd7b656acd6)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(clippy::approx_constant, clippy::type_complexity, clippy::unreadable_literal, clippy::upper_case_acronyms)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use glib_sys as glib;
use gobject_sys as gobject;
use gio_sys as gio;
use gtk_sys as gtk;
use gdk_sys as gdk;
use gdk_pixbuf_sys as gdk_pixbuf;
use pango_sys as pango;
use handy_sys as handy;

#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, time_t, off_t, intptr_t, uintptr_t, FILE};
#[cfg(unix)]
#[allow(unused_imports)]
use libc::{dev_t, gid_t, pid_t, socklen_t, uid_t};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type PhoshLockscreenPage = c_int;
pub const PHOSH_LOCKSCREEN_PAGE_INFO: PhoshLockscreenPage = 0;
pub const PHOSH_LOCKSCREEN_PAGE_EXTRA: PhoshLockscreenPage = 1;
pub const PHOSH_LOCKSCREEN_PAGE_UNLOCK: PhoshLockscreenPage = 2;

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhoshDBusScreenshotProxyClass {
    pub parent_class: gio::GDBusProxyClass,
}

impl ::std::fmt::Debug for PhoshDBusScreenshotProxyClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PhoshDBusScreenshotProxyClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[repr(C)]
pub struct _PhoshDBusScreenshotProxyPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type PhoshDBusScreenshotProxyPrivate = _PhoshDBusScreenshotProxyPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhoshDBusScreenshotSkeletonClass {
    pub parent_class: gio::GDBusInterfaceSkeletonClass,
}

impl ::std::fmt::Debug for PhoshDBusScreenshotSkeletonClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PhoshDBusScreenshotSkeletonClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[repr(C)]
pub struct _PhoshDBusScreenshotSkeletonPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type PhoshDBusScreenshotSkeletonPrivate = _PhoshDBusScreenshotSkeletonPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhoshLayerSurfaceClass {
    pub parent_class: gtk::GtkWindowClass,
    pub configured: Option<unsafe extern "C" fn(*mut PhoshLayerSurface)>,
    pub _phosh_reserved1: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved2: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved3: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved4: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved5: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved6: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved7: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved8: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved9: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for PhoshLayerSurfaceClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PhoshLayerSurfaceClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .field("configured", &self.configured)
         .field("_phosh_reserved1", &self._phosh_reserved1)
         .field("_phosh_reserved2", &self._phosh_reserved2)
         .field("_phosh_reserved3", &self._phosh_reserved3)
         .field("_phosh_reserved4", &self._phosh_reserved4)
         .field("_phosh_reserved5", &self._phosh_reserved5)
         .field("_phosh_reserved6", &self._phosh_reserved6)
         .field("_phosh_reserved7", &self._phosh_reserved7)
         .field("_phosh_reserved8", &self._phosh_reserved8)
         .field("_phosh_reserved9", &self._phosh_reserved9)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhoshLockscreenClass {
    pub parent_class: PhoshLayerSurfaceClass,
    pub unlock_submit: Option<unsafe extern "C" fn(*mut PhoshLockscreen)>,
    pub _phosh_reserved1: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved2: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved3: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved4: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved5: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved6: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved7: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved8: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved9: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for PhoshLockscreenClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PhoshLockscreenClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .field("unlock_submit", &self.unlock_submit)
         .field("_phosh_reserved1", &self._phosh_reserved1)
         .field("_phosh_reserved2", &self._phosh_reserved2)
         .field("_phosh_reserved3", &self._phosh_reserved3)
         .field("_phosh_reserved4", &self._phosh_reserved4)
         .field("_phosh_reserved5", &self._phosh_reserved5)
         .field("_phosh_reserved6", &self._phosh_reserved6)
         .field("_phosh_reserved7", &self._phosh_reserved7)
         .field("_phosh_reserved8", &self._phosh_reserved8)
         .field("_phosh_reserved9", &self._phosh_reserved9)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhoshLockscreenManagerClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for PhoshLockscreenManagerClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PhoshLockscreenManagerClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhoshQuickSettingClass {
    pub parent_class: gtk::GtkBoxClass,
    pub _phosh_reserved0: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved1: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved2: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved3: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved4: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved5: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved6: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved7: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved8: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved9: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for PhoshQuickSettingClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PhoshQuickSettingClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .field("_phosh_reserved0", &self._phosh_reserved0)
         .field("_phosh_reserved1", &self._phosh_reserved1)
         .field("_phosh_reserved2", &self._phosh_reserved2)
         .field("_phosh_reserved3", &self._phosh_reserved3)
         .field("_phosh_reserved4", &self._phosh_reserved4)
         .field("_phosh_reserved5", &self._phosh_reserved5)
         .field("_phosh_reserved6", &self._phosh_reserved6)
         .field("_phosh_reserved7", &self._phosh_reserved7)
         .field("_phosh_reserved8", &self._phosh_reserved8)
         .field("_phosh_reserved9", &self._phosh_reserved9)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhoshScreenshotManagerClass {
    pub parent_class: PhoshDBusScreenshotSkeletonClass,
}

impl ::std::fmt::Debug for PhoshScreenshotManagerClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PhoshScreenshotManagerClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhoshShellClass {
    pub parent_class: gobject::GObjectClass,
    pub get_lockscreen_type: Option<unsafe extern "C" fn(*mut PhoshShell) -> GType>,
    pub _phosh_reserved1: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved2: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved3: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved4: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved5: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved6: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved7: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved8: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved9: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for PhoshShellClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PhoshShellClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .field("get_lockscreen_type", &self.get_lockscreen_type)
         .field("_phosh_reserved1", &self._phosh_reserved1)
         .field("_phosh_reserved2", &self._phosh_reserved2)
         .field("_phosh_reserved3", &self._phosh_reserved3)
         .field("_phosh_reserved4", &self._phosh_reserved4)
         .field("_phosh_reserved5", &self._phosh_reserved5)
         .field("_phosh_reserved6", &self._phosh_reserved6)
         .field("_phosh_reserved7", &self._phosh_reserved7)
         .field("_phosh_reserved8", &self._phosh_reserved8)
         .field("_phosh_reserved9", &self._phosh_reserved9)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhoshStatusIconClass {
    pub parent_class: gtk::GtkBinClass,
    pub idle_init: Option<unsafe extern "C" fn(*mut PhoshStatusIcon)>,
    pub _phosh_reserved1: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved2: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved3: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved4: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved5: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved6: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved7: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved8: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved9: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for PhoshStatusIconClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PhoshStatusIconClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .field("idle_init", &self.idle_init)
         .field("_phosh_reserved1", &self._phosh_reserved1)
         .field("_phosh_reserved2", &self._phosh_reserved2)
         .field("_phosh_reserved3", &self._phosh_reserved3)
         .field("_phosh_reserved4", &self._phosh_reserved4)
         .field("_phosh_reserved5", &self._phosh_reserved5)
         .field("_phosh_reserved6", &self._phosh_reserved6)
         .field("_phosh_reserved7", &self._phosh_reserved7)
         .field("_phosh_reserved8", &self._phosh_reserved8)
         .field("_phosh_reserved9", &self._phosh_reserved9)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhoshStatusPageClass {
    pub parent_class: gtk::GtkBinClass,
    pub _phosh_reserved0: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved1: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved2: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved3: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved4: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved5: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved6: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved7: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved8: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved9: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for PhoshStatusPageClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PhoshStatusPageClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .field("_phosh_reserved0", &self._phosh_reserved0)
         .field("_phosh_reserved1", &self._phosh_reserved1)
         .field("_phosh_reserved2", &self._phosh_reserved2)
         .field("_phosh_reserved3", &self._phosh_reserved3)
         .field("_phosh_reserved4", &self._phosh_reserved4)
         .field("_phosh_reserved5", &self._phosh_reserved5)
         .field("_phosh_reserved6", &self._phosh_reserved6)
         .field("_phosh_reserved7", &self._phosh_reserved7)
         .field("_phosh_reserved8", &self._phosh_reserved8)
         .field("_phosh_reserved9", &self._phosh_reserved9)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhoshWallClockClass {
    pub parent_class: gobject::GObjectClass,
    pub get_clock: Option<unsafe extern "C" fn(*mut PhoshWallClock, gboolean) -> *const c_char>,
    pub get_time_t: Option<unsafe extern "C" fn(*mut PhoshWallClock) -> i64>,
    pub _phosh_reserved2: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved3: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved4: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved5: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved6: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved7: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved8: Option<unsafe extern "C" fn()>,
    pub _phosh_reserved9: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for PhoshWallClockClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PhoshWallClockClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .field("get_clock", &self.get_clock)
         .field("get_time_t", &self.get_time_t)
         .field("_phosh_reserved2", &self._phosh_reserved2)
         .field("_phosh_reserved3", &self._phosh_reserved3)
         .field("_phosh_reserved4", &self._phosh_reserved4)
         .field("_phosh_reserved5", &self._phosh_reserved5)
         .field("_phosh_reserved6", &self._phosh_reserved6)
         .field("_phosh_reserved7", &self._phosh_reserved7)
         .field("_phosh_reserved8", &self._phosh_reserved8)
         .field("_phosh_reserved9", &self._phosh_reserved9)
         .finish()
    }
}

// Classes
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhoshLayerSurface {
    pub parent_instance: gtk::GtkWindow,
}

impl ::std::fmt::Debug for PhoshLayerSurface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PhoshLayerSurface @ {self:p}"))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhoshLockscreen {
    pub parent_instance: PhoshLayerSurface,
}

impl ::std::fmt::Debug for PhoshLockscreen {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PhoshLockscreen @ {self:p}"))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[repr(C)]
pub struct PhoshLockscreenManager {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for PhoshLockscreenManager {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PhoshLockscreenManager @ {self:p}"))
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhoshQuickSetting {
    pub parent_instance: gtk::GtkBox,
}

impl ::std::fmt::Debug for PhoshQuickSetting {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PhoshQuickSetting @ {self:p}"))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[repr(C)]
pub struct PhoshScreenshotManager {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for PhoshScreenshotManager {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PhoshScreenshotManager @ {self:p}"))
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhoshShell {
    pub parent_instance: gobject::GObject,
}

impl ::std::fmt::Debug for PhoshShell {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PhoshShell @ {self:p}"))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhoshStatusIcon {
    pub parent_instance: gtk::GtkBin,
}

impl ::std::fmt::Debug for PhoshStatusIcon {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PhoshStatusIcon @ {self:p}"))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhoshStatusPage {
    pub parent_instance: gtk::GtkBin,
}

impl ::std::fmt::Debug for PhoshStatusPage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PhoshStatusPage @ {self:p}"))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhoshWallClock {
    pub parent_instance: gobject::GObject,
}

impl ::std::fmt::Debug for PhoshWallClock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PhoshWallClock @ {self:p}"))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

// Interfaces

#[link(name = "phosh-0.45")]
extern "C" {

    //=========================================================================
    // PhoshLockscreenPage
    //=========================================================================
    pub fn phosh_lockscreen_page_get_type() -> GType;

    //=========================================================================
    // PhoshDBusScreenshotProxy
    //=========================================================================
    pub fn phosh_dbus_screenshot_proxy_get_type() -> GType;
    //pub fn phosh_dbus_screenshot_proxy_new_finish(res: *mut gio::GAsyncResult, error: *mut *mut glib::GError) -> /*Ignored*/*mut PhoshDBusScreenshotProxy;
    //pub fn phosh_dbus_screenshot_proxy_new_for_bus_finish(res: *mut gio::GAsyncResult, error: *mut *mut glib::GError) -> /*Ignored*/*mut PhoshDBusScreenshotProxy;
    //pub fn phosh_dbus_screenshot_proxy_new_for_bus_sync(bus_type: gio::GBusType, flags: gio::GDBusProxyFlags, name: *const c_char, object_path: *const c_char, cancellable: *mut gio::GCancellable, error: *mut *mut glib::GError) -> /*Ignored*/*mut PhoshDBusScreenshotProxy;
    //pub fn phosh_dbus_screenshot_proxy_new_sync(connection: *mut gio::GDBusConnection, flags: gio::GDBusProxyFlags, name: *const c_char, object_path: *const c_char, cancellable: *mut gio::GCancellable, error: *mut *mut glib::GError) -> /*Ignored*/*mut PhoshDBusScreenshotProxy;
    pub fn phosh_dbus_screenshot_proxy_new(connection: *mut gio::GDBusConnection, flags: gio::GDBusProxyFlags, name: *const c_char, object_path: *const c_char, cancellable: *mut gio::GCancellable, callback: gio::GAsyncReadyCallback, user_data: gpointer);
    pub fn phosh_dbus_screenshot_proxy_new_for_bus(bus_type: gio::GBusType, flags: gio::GDBusProxyFlags, name: *const c_char, object_path: *const c_char, cancellable: *mut gio::GCancellable, callback: gio::GAsyncReadyCallback, user_data: gpointer);

    //=========================================================================
    // PhoshDBusScreenshotSkeleton
    //=========================================================================
    pub fn phosh_dbus_screenshot_skeleton_get_type() -> GType;
    //pub fn phosh_dbus_screenshot_skeleton_new() -> /*Ignored*/*mut PhoshDBusScreenshotSkeleton;

    //=========================================================================
    // PhoshLayerSurface
    //=========================================================================
    pub fn phosh_layer_surface_get_type() -> GType;

    //=========================================================================
    // PhoshLockscreen
    //=========================================================================
    pub fn phosh_lockscreen_get_type() -> GType;
    pub fn phosh_lockscreen_add_extra_page(self_: *mut PhoshLockscreen, widget: *mut gtk::GtkWidget);
    pub fn phosh_lockscreen_clear_pin_entry(self_: *mut PhoshLockscreen);
    pub fn phosh_lockscreen_get_page(self_: *mut PhoshLockscreen) -> PhoshLockscreenPage;
    pub fn phosh_lockscreen_get_pin_entry(self_: *mut PhoshLockscreen) -> *const c_char;
    pub fn phosh_lockscreen_set_default_page(self_: *mut PhoshLockscreen, page: PhoshLockscreenPage);
    pub fn phosh_lockscreen_set_page(self_: *mut PhoshLockscreen, page: PhoshLockscreenPage);
    pub fn phosh_lockscreen_set_unlock_status(self_: *mut PhoshLockscreen, status: *const c_char);
    pub fn phosh_lockscreen_shake_pin_entry(self_: *mut PhoshLockscreen);

    //=========================================================================
    // PhoshLockscreenManager
    //=========================================================================
    pub fn phosh_lockscreen_manager_get_type() -> GType;
    pub fn phosh_lockscreen_manager_get_active_time(self_: *mut PhoshLockscreenManager) -> i64;
    pub fn phosh_lockscreen_manager_get_locked(self_: *mut PhoshLockscreenManager) -> gboolean;
    pub fn phosh_lockscreen_manager_get_lockscreen(self_: *mut PhoshLockscreenManager) -> *mut PhoshLockscreen;
    pub fn phosh_lockscreen_manager_get_page(self_: *mut PhoshLockscreenManager) -> PhoshLockscreenPage;
    pub fn phosh_lockscreen_manager_set_locked(self_: *mut PhoshLockscreenManager, state: gboolean);
    pub fn phosh_lockscreen_manager_set_page(self_: *mut PhoshLockscreenManager, page: PhoshLockscreenPage) -> gboolean;

    //=========================================================================
    // PhoshQuickSetting
    //=========================================================================
    pub fn phosh_quick_setting_get_type() -> GType;
    pub fn phosh_quick_setting_new(status_page: *mut PhoshStatusPage) -> *mut gtk::GtkWidget;
    pub fn phosh_quick_setting_get_active(self_: *mut PhoshQuickSetting) -> gboolean;
    pub fn phosh_quick_setting_get_can_show_status(self_: *mut PhoshQuickSetting) -> gboolean;
    pub fn phosh_quick_setting_get_long_press_action_name(self_: *mut PhoshQuickSetting) -> *const c_char;
    pub fn phosh_quick_setting_get_long_press_action_target(self_: *mut PhoshQuickSetting) -> *const c_char;
    pub fn phosh_quick_setting_get_showing_status(self_: *mut PhoshQuickSetting) -> gboolean;
    pub fn phosh_quick_setting_get_status_page(self_: *mut PhoshQuickSetting) -> *mut PhoshStatusPage;
    pub fn phosh_quick_setting_set_active(self_: *mut PhoshQuickSetting, active: gboolean);
    pub fn phosh_quick_setting_set_can_show_status(self_: *mut PhoshQuickSetting, can_show_status: gboolean);
    pub fn phosh_quick_setting_set_long_press_action_name(self_: *mut PhoshQuickSetting, action_name: *const c_char);
    pub fn phosh_quick_setting_set_long_press_action_target(self_: *mut PhoshQuickSetting, action_target: *const c_char);
    pub fn phosh_quick_setting_set_showing_status(self_: *mut PhoshQuickSetting, showing_status: gboolean);
    pub fn phosh_quick_setting_set_status_page(self_: *mut PhoshQuickSetting, status_page: *mut PhoshStatusPage);

    //=========================================================================
    // PhoshScreenshotManager
    //=========================================================================
    pub fn phosh_screenshot_manager_get_type() -> GType;
    pub fn phosh_screenshot_manager_new() -> *mut PhoshScreenshotManager;
    pub fn phosh_screenshot_manager_take_screenshot(self_: *mut PhoshScreenshotManager, area: *const gdk::GdkRectangle, filename: *const c_char, copy_to_clipboard: gboolean, include_cursor: gboolean) -> gboolean;

    //=========================================================================
    // PhoshShell
    //=========================================================================
    pub fn phosh_shell_get_type() -> GType;
    pub fn phosh_shell_new() -> *mut PhoshShell;
    pub fn phosh_shell_get_default() -> *mut PhoshShell;
    pub fn phosh_shell_fade_out(self_: *mut PhoshShell, timeout: c_uint);
    pub fn phosh_shell_get_locked(self_: *mut PhoshShell) -> gboolean;
    pub fn phosh_shell_get_lockscreen_manager(self_: *mut PhoshShell) -> *mut PhoshLockscreenManager;
    pub fn phosh_shell_get_lockscreen_type(self_: *mut PhoshShell) -> GType;
    pub fn phosh_shell_get_screenshot_manager(self_: *mut PhoshShell) -> *mut PhoshScreenshotManager;
    pub fn phosh_shell_get_usable_area(self_: *mut PhoshShell, x: *mut c_int, y: *mut c_int, width: *mut c_int, height: *mut c_int);
    pub fn phosh_shell_set_default(self_: *mut PhoshShell);

    //=========================================================================
    // PhoshStatusIcon
    //=========================================================================
    pub fn phosh_status_icon_get_type() -> GType;
    pub fn phosh_status_icon_new() -> *mut gtk::GtkWidget;
    pub fn phosh_status_icon_get_extra_widget(self_: *mut PhoshStatusIcon) -> *mut gtk::GtkWidget;
    pub fn phosh_status_icon_get_icon_name(self_: *mut PhoshStatusIcon) -> *mut c_char;
    pub fn phosh_status_icon_get_icon_size(self_: *mut PhoshStatusIcon) -> gtk::GtkIconSize;
    pub fn phosh_status_icon_get_info(self_: *mut PhoshStatusIcon) -> *mut c_char;
    pub fn phosh_status_icon_get_show_always(self_: *mut PhoshStatusIcon) -> gboolean;
    pub fn phosh_status_icon_set_extra_widget(self_: *mut PhoshStatusIcon, widget: *mut gtk::GtkWidget);
    pub fn phosh_status_icon_set_icon_name(self_: *mut PhoshStatusIcon, icon_name: *const c_char);
    pub fn phosh_status_icon_set_icon_size(self_: *mut PhoshStatusIcon, size: gtk::GtkIconSize);
    pub fn phosh_status_icon_set_info(self_: *mut PhoshStatusIcon, info: *const c_char);
    pub fn phosh_status_icon_set_show_always(self_: *mut PhoshStatusIcon, show_always: gboolean);

    //=========================================================================
    // PhoshStatusPage
    //=========================================================================
    pub fn phosh_status_page_get_type() -> GType;
    pub fn phosh_status_page_new() -> *mut PhoshStatusPage;
    pub fn phosh_status_page_get_footer(self_: *mut PhoshStatusPage) -> *mut gtk::GtkWidget;
    pub fn phosh_status_page_get_header(self_: *mut PhoshStatusPage) -> *mut gtk::GtkWidget;
    pub fn phosh_status_page_get_title(self_: *mut PhoshStatusPage) -> *const c_char;
    pub fn phosh_status_page_set_footer(self_: *mut PhoshStatusPage, footer: *mut gtk::GtkWidget);
    pub fn phosh_status_page_set_header(self_: *mut PhoshStatusPage, header: *mut gtk::GtkWidget);
    pub fn phosh_status_page_set_title(self_: *mut PhoshStatusPage, title: *const c_char);

    //=========================================================================
    // PhoshWallClock
    //=========================================================================
    pub fn phosh_wall_clock_get_type() -> GType;
    pub fn phosh_wall_clock_new() -> *mut PhoshWallClock;
    pub fn phosh_wall_clock_get_default() -> *mut PhoshWallClock;
    pub fn phosh_wall_clock_get_clock(self_: *mut PhoshWallClock, time_only: gboolean) -> *const c_char;
    pub fn phosh_wall_clock_local_date(self_: *mut PhoshWallClock) -> *mut c_char;
    pub fn phosh_wall_clock_set_default(self_: *mut PhoshWallClock);

    //=========================================================================
    // PhoshDBusScreenshot
    //=========================================================================
    pub fn phosh_dbus_screenshot_get_type() -> GType;
    pub fn phosh_dbus_screenshot_interface_info() -> *mut gio::GDBusInterfaceInfo;
    pub fn phosh_dbus_screenshot_override_properties(klass: *mut gobject::GObjectClass, property_id_begin: c_uint) -> c_uint;
    //pub fn phosh_dbus_screenshot_call_flash_area(proxy: /*Ignored*/*mut PhoshDBusScreenshot, arg_x: c_int, arg_y: c_int, arg_width: c_int, arg_height: c_int, cancellable: *mut gio::GCancellable, callback: gio::GAsyncReadyCallback, user_data: gpointer);
    //pub fn phosh_dbus_screenshot_call_flash_area_finish(proxy: /*Ignored*/*mut PhoshDBusScreenshot, res: *mut gio::GAsyncResult, error: *mut *mut glib::GError) -> gboolean;
    //pub fn phosh_dbus_screenshot_call_flash_area_sync(proxy: /*Ignored*/*mut PhoshDBusScreenshot, arg_x: c_int, arg_y: c_int, arg_width: c_int, arg_height: c_int, cancellable: *mut gio::GCancellable, error: *mut *mut glib::GError) -> gboolean;
    //pub fn phosh_dbus_screenshot_call_pick_color(proxy: /*Ignored*/*mut PhoshDBusScreenshot, cancellable: *mut gio::GCancellable, callback: gio::GAsyncReadyCallback, user_data: gpointer);
    //pub fn phosh_dbus_screenshot_call_pick_color_finish(proxy: /*Ignored*/*mut PhoshDBusScreenshot, out_result: *mut *mut glib::GVariant, res: *mut gio::GAsyncResult, error: *mut *mut glib::GError) -> gboolean;
    //pub fn phosh_dbus_screenshot_call_pick_color_sync(proxy: /*Ignored*/*mut PhoshDBusScreenshot, out_result: *mut *mut glib::GVariant, cancellable: *mut gio::GCancellable, error: *mut *mut glib::GError) -> gboolean;
    //pub fn phosh_dbus_screenshot_call_screenshot(proxy: /*Ignored*/*mut PhoshDBusScreenshot, arg_include_cursor: gboolean, arg_flash: gboolean, arg_filename: *const c_char, cancellable: *mut gio::GCancellable, callback: gio::GAsyncReadyCallback, user_data: gpointer);
    //pub fn phosh_dbus_screenshot_call_screenshot_area(proxy: /*Ignored*/*mut PhoshDBusScreenshot, arg_x: c_int, arg_y: c_int, arg_width: c_int, arg_height: c_int, arg_flash: gboolean, arg_filename: *const c_char, cancellable: *mut gio::GCancellable, callback: gio::GAsyncReadyCallback, user_data: gpointer);
    //pub fn phosh_dbus_screenshot_call_screenshot_area_finish(proxy: /*Ignored*/*mut PhoshDBusScreenshot, out_success: *mut gboolean, out_filename_used: *mut *mut c_char, res: *mut gio::GAsyncResult, error: *mut *mut glib::GError) -> gboolean;
    //pub fn phosh_dbus_screenshot_call_screenshot_area_sync(proxy: /*Ignored*/*mut PhoshDBusScreenshot, arg_x: c_int, arg_y: c_int, arg_width: c_int, arg_height: c_int, arg_flash: gboolean, arg_filename: *const c_char, out_success: *mut gboolean, out_filename_used: *mut *mut c_char, cancellable: *mut gio::GCancellable, error: *mut *mut glib::GError) -> gboolean;
    //pub fn phosh_dbus_screenshot_call_screenshot_finish(proxy: /*Ignored*/*mut PhoshDBusScreenshot, out_success: *mut gboolean, out_filename_used: *mut *mut c_char, res: *mut gio::GAsyncResult, error: *mut *mut glib::GError) -> gboolean;
    //pub fn phosh_dbus_screenshot_call_screenshot_sync(proxy: /*Ignored*/*mut PhoshDBusScreenshot, arg_include_cursor: gboolean, arg_flash: gboolean, arg_filename: *const c_char, out_success: *mut gboolean, out_filename_used: *mut *mut c_char, cancellable: *mut gio::GCancellable, error: *mut *mut glib::GError) -> gboolean;
    //pub fn phosh_dbus_screenshot_call_screenshot_window(proxy: /*Ignored*/*mut PhoshDBusScreenshot, arg_include_frame: gboolean, arg_include_cursor: gboolean, arg_flash: gboolean, arg_filename: *const c_char, cancellable: *mut gio::GCancellable, callback: gio::GAsyncReadyCallback, user_data: gpointer);
    //pub fn phosh_dbus_screenshot_call_screenshot_window_finish(proxy: /*Ignored*/*mut PhoshDBusScreenshot, out_success: *mut gboolean, out_filename_used: *mut *mut c_char, res: *mut gio::GAsyncResult, error: *mut *mut glib::GError) -> gboolean;
    //pub fn phosh_dbus_screenshot_call_screenshot_window_sync(proxy: /*Ignored*/*mut PhoshDBusScreenshot, arg_include_frame: gboolean, arg_include_cursor: gboolean, arg_flash: gboolean, arg_filename: *const c_char, out_success: *mut gboolean, out_filename_used: *mut *mut c_char, cancellable: *mut gio::GCancellable, error: *mut *mut glib::GError) -> gboolean;
    //pub fn phosh_dbus_screenshot_call_select_area(proxy: /*Ignored*/*mut PhoshDBusScreenshot, cancellable: *mut gio::GCancellable, callback: gio::GAsyncReadyCallback, user_data: gpointer);
    //pub fn phosh_dbus_screenshot_call_select_area_finish(proxy: /*Ignored*/*mut PhoshDBusScreenshot, out_x: *mut c_int, out_y: *mut c_int, out_width: *mut c_int, out_height: *mut c_int, res: *mut gio::GAsyncResult, error: *mut *mut glib::GError) -> gboolean;
    //pub fn phosh_dbus_screenshot_call_select_area_sync(proxy: /*Ignored*/*mut PhoshDBusScreenshot, out_x: *mut c_int, out_y: *mut c_int, out_width: *mut c_int, out_height: *mut c_int, cancellable: *mut gio::GCancellable, error: *mut *mut glib::GError) -> gboolean;
    //pub fn phosh_dbus_screenshot_complete_flash_area(object: /*Ignored*/*mut PhoshDBusScreenshot, invocation: *mut gio::GDBusMethodInvocation);
    //pub fn phosh_dbus_screenshot_complete_pick_color(object: /*Ignored*/*mut PhoshDBusScreenshot, invocation: *mut gio::GDBusMethodInvocation, result: *mut glib::GVariant);
    //pub fn phosh_dbus_screenshot_complete_screenshot(object: /*Ignored*/*mut PhoshDBusScreenshot, invocation: *mut gio::GDBusMethodInvocation, success: gboolean, filename_used: *const c_char);
    //pub fn phosh_dbus_screenshot_complete_screenshot_area(object: /*Ignored*/*mut PhoshDBusScreenshot, invocation: *mut gio::GDBusMethodInvocation, success: gboolean, filename_used: *const c_char);
    //pub fn phosh_dbus_screenshot_complete_screenshot_window(object: /*Ignored*/*mut PhoshDBusScreenshot, invocation: *mut gio::GDBusMethodInvocation, success: gboolean, filename_used: *const c_char);
    //pub fn phosh_dbus_screenshot_complete_select_area(object: /*Ignored*/*mut PhoshDBusScreenshot, invocation: *mut gio::GDBusMethodInvocation, x: c_int, y: c_int, width: c_int, height: c_int);

}
