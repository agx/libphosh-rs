// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "PhoshStatusIcon")]
    pub struct StatusIcon(Object<ffi::PhoshStatusIcon, ffi::PhoshStatusIconClass>) @extends gtk::Bin, gtk::Container, gtk::Widget;

    match fn {
        type_ => || ffi::phosh_status_icon_get_type(),
    }
}

impl StatusIcon {
        pub const NONE: Option<&'static StatusIcon> = None;
    

    #[doc(alias = "phosh_status_icon_new")]
    pub fn new() -> StatusIcon {
        assert_initialized_main_thread!();
        unsafe {
            gtk::Widget::from_glib_none(ffi::phosh_status_icon_new()).unsafe_cast()
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`StatusIcon`] objects.
            ///
            /// This method returns an instance of [`StatusIconBuilder`](crate::builders::StatusIconBuilder) which can be used to create [`StatusIcon`] objects.
            pub fn builder() -> StatusIconBuilder {
                StatusIconBuilder::new()
            }
        
}

impl Default for StatusIcon {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`StatusIcon`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct StatusIconBuilder {
            builder: glib::object::ObjectBuilder<'static, StatusIcon>,
        }

        impl StatusIconBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn extra_widget(self, extra_widget: &impl IsA<gtk::Widget>) -> Self {
                            Self { builder: self.builder.property("extra-widget", extra_widget.clone().upcast()), }
                        }

                            pub fn icon_name(self, icon_name: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("icon-name", icon_name.into()), }
                        }

                            //pub fn icon_size(self, icon_size: /*Ignored*/gtk::IconSize) -> Self {
                        //    Self { builder: self.builder.property("icon-size", icon_size), }
                        //}

                            pub fn info(self, info: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("info", info.into()), }
                        }

                            pub fn border_width(self, border_width: u32) -> Self {
                            Self { builder: self.builder.property("border-width", border_width), }
                        }

                            pub fn child(self, child: &impl IsA<gtk::Widget>) -> Self {
                            Self { builder: self.builder.property("child", child.clone().upcast()), }
                        }

                            //pub fn resize_mode(self, resize_mode: /*Ignored*/gtk::ResizeMode) -> Self {
                        //    Self { builder: self.builder.property("resize-mode", resize_mode), }
                        //}

                            pub fn app_paintable(self, app_paintable: bool) -> Self {
                            Self { builder: self.builder.property("app-paintable", app_paintable), }
                        }

                            pub fn can_default(self, can_default: bool) -> Self {
                            Self { builder: self.builder.property("can-default", can_default), }
                        }

                            pub fn can_focus(self, can_focus: bool) -> Self {
                            Self { builder: self.builder.property("can-focus", can_focus), }
                        }

                            #[cfg(feature = "gtk_v2_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v2_18")))]
    #[cfg_attr(feature = "v3_14", deprecated = "Since 3.14")]
    pub fn double_buffered(self, double_buffered: bool) -> Self {
                            Self { builder: self.builder.property("double-buffered", double_buffered), }
                        }

                            //pub fn events(self, events: /*Ignored*/gdk::EventMask) -> Self {
                        //    Self { builder: self.builder.property("events", events), }
                        //}

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    pub fn expand(self, expand: bool) -> Self {
                            Self { builder: self.builder.property("expand", expand), }
                        }

                            #[cfg(feature = "gtk_v3_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3_20")))]
    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
                            Self { builder: self.builder.property("focus-on-click", focus_on_click), }
                        }

                        //    #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    //pub fn halign(self, halign: /*Ignored*/gtk::Align) -> Self {
                        //    Self { builder: self.builder.property("halign", halign), }
                        //}

                            pub fn has_default(self, has_default: bool) -> Self {
                            Self { builder: self.builder.property("has-default", has_default), }
                        }

                            pub fn has_focus(self, has_focus: bool) -> Self {
                            Self { builder: self.builder.property("has-focus", has_focus), }
                        }

                            #[cfg(feature = "gtk_v2_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v2_12")))]
    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
                            Self { builder: self.builder.property("has-tooltip", has_tooltip), }
                        }

                            pub fn height_request(self, height_request: i32) -> Self {
                            Self { builder: self.builder.property("height-request", height_request), }
                        }

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    pub fn hexpand(self, hexpand: bool) -> Self {
                            Self { builder: self.builder.property("hexpand", hexpand), }
                        }

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
                            Self { builder: self.builder.property("hexpand-set", hexpand_set), }
                        }

                            pub fn is_focus(self, is_focus: bool) -> Self {
                            Self { builder: self.builder.property("is-focus", is_focus), }
                        }

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    pub fn margin(self, margin: i32) -> Self {
                            Self { builder: self.builder.property("margin", margin), }
                        }

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
                            Self { builder: self.builder.property("margin-bottom", margin_bottom), }
                        }

                            #[cfg(feature = "gtk_v3_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3_12")))]
    pub fn margin_end(self, margin_end: i32) -> Self {
                            Self { builder: self.builder.property("margin-end", margin_end), }
                        }

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    #[cfg_attr(feature = "v3_12", deprecated = "Since 3.12")]
    pub fn margin_left(self, margin_left: i32) -> Self {
                            Self { builder: self.builder.property("margin-left", margin_left), }
                        }

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    #[cfg_attr(feature = "v3_12", deprecated = "Since 3.12")]
    pub fn margin_right(self, margin_right: i32) -> Self {
                            Self { builder: self.builder.property("margin-right", margin_right), }
                        }

                            #[cfg(feature = "gtk_v3_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3_12")))]
    pub fn margin_start(self, margin_start: i32) -> Self {
                            Self { builder: self.builder.property("margin-start", margin_start), }
                        }

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    pub fn margin_top(self, margin_top: i32) -> Self {
                            Self { builder: self.builder.property("margin-top", margin_top), }
                        }

                            pub fn name(self, name: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("name", name.into()), }
                        }

                            pub fn no_show_all(self, no_show_all: bool) -> Self {
                            Self { builder: self.builder.property("no-show-all", no_show_all), }
                        }

                            #[cfg(feature = "gtk_v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3_8")))]
    pub fn opacity(self, opacity: f64) -> Self {
                            Self { builder: self.builder.property("opacity", opacity), }
                        }

                            pub fn parent(self, parent: &impl IsA<gtk::Container>) -> Self {
                            Self { builder: self.builder.property("parent", parent.clone().upcast()), }
                        }

                            pub fn receives_default(self, receives_default: bool) -> Self {
                            Self { builder: self.builder.property("receives-default", receives_default), }
                        }

                            pub fn sensitive(self, sensitive: bool) -> Self {
                            Self { builder: self.builder.property("sensitive", sensitive), }
                        }

                            //pub fn style(self, style: &impl IsA</*Ignored*/gtk::Style>) -> Self {
                        //    Self { builder: self.builder.property("style", style.clone().upcast()), }
                        //}

                            #[cfg(feature = "gtk_v2_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v2_12")))]
    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("tooltip-markup", tooltip_markup.into()), }
                        }

                            #[cfg(feature = "gtk_v2_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v2_12")))]
    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("tooltip-text", tooltip_text.into()), }
                        }

                        //    #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    //pub fn valign(self, valign: /*Ignored*/gtk::Align) -> Self {
                        //    Self { builder: self.builder.property("valign", valign), }
                        //}

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    pub fn vexpand(self, vexpand: bool) -> Self {
                            Self { builder: self.builder.property("vexpand", vexpand), }
                        }

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
                            Self { builder: self.builder.property("vexpand-set", vexpand_set), }
                        }

                            pub fn visible(self, visible: bool) -> Self {
                            Self { builder: self.builder.property("visible", visible), }
                        }

                            pub fn width_request(self, width_request: i32) -> Self {
                            Self { builder: self.builder.property("width-request", width_request), }
                        }

    // rustdoc-stripper-ignore-next
    /// Build the [`StatusIcon`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> StatusIcon {
    self.builder.build() }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::StatusIcon>> Sealed for T {}
}

pub trait StatusIconExt: IsA<StatusIcon> + sealed::Sealed + 'static {
    #[doc(alias = "phosh_status_icon_get_extra_widget")]
    #[doc(alias = "get_extra_widget")]
    fn extra_widget(&self) -> Option<gtk::Widget> {
        unsafe {
            from_glib_none(ffi::phosh_status_icon_get_extra_widget(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "phosh_status_icon_get_icon_name")]
    #[doc(alias = "get_icon_name")]
    fn icon_name(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::phosh_status_icon_get_icon_name(self.as_ref().to_glib_none().0))
        }
    }

    //#[doc(alias = "phosh_status_icon_get_icon_size")]
    //#[doc(alias = "get_icon_size")]
    //fn icon_size(&self) -> /*Ignored*/gtk::IconSize {
    //    unsafe { TODO: call ffi:phosh_status_icon_get_icon_size() }
    //}

    #[doc(alias = "phosh_status_icon_get_info")]
    #[doc(alias = "get_info")]
    fn info(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::phosh_status_icon_get_info(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "phosh_status_icon_get_show_always")]
    #[doc(alias = "get_show_always")]
    fn shows_always(&self) -> bool {
        unsafe {
            from_glib(ffi::phosh_status_icon_get_show_always(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "phosh_status_icon_set_extra_widget")]
    fn set_extra_widget(&self, widget: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::phosh_status_icon_set_extra_widget(self.as_ref().to_glib_none().0, widget.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "phosh_status_icon_set_icon_name")]
    fn set_icon_name(&self, icon_name: &str) {
        unsafe {
            ffi::phosh_status_icon_set_icon_name(self.as_ref().to_glib_none().0, icon_name.to_glib_none().0);
        }
    }

    //#[doc(alias = "phosh_status_icon_set_icon_size")]
    //fn set_icon_size(&self, size: /*Ignored*/gtk::IconSize) {
    //    unsafe { TODO: call ffi:phosh_status_icon_set_icon_size() }
    //}

    #[doc(alias = "phosh_status_icon_set_info")]
    fn set_info(&self, info: &str) {
        unsafe {
            ffi::phosh_status_icon_set_info(self.as_ref().to_glib_none().0, info.to_glib_none().0);
        }
    }

    #[doc(alias = "phosh_status_icon_set_show_always")]
    fn set_show_always(&self, show_always: bool) {
        unsafe {
            ffi::phosh_status_icon_set_show_always(self.as_ref().to_glib_none().0, show_always.into_glib());
        }
    }

    #[doc(alias = "extra-widget")]
    fn connect_extra_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_extra_widget_trampoline<P: IsA<StatusIcon>, F: Fn(&P) + 'static>(this: *mut ffi::PhoshStatusIcon, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::extra-widget\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_extra_widget_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "icon-name")]
    fn connect_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<P: IsA<StatusIcon>, F: Fn(&P) + 'static>(this: *mut ffi::PhoshStatusIcon, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::icon-name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_icon_name_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "icon-size")]
    fn connect_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_size_trampoline<P: IsA<StatusIcon>, F: Fn(&P) + 'static>(this: *mut ffi::PhoshStatusIcon, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::icon-size\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_icon_size_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "info")]
    fn connect_info_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_info_trampoline<P: IsA<StatusIcon>, F: Fn(&P) + 'static>(this: *mut ffi::PhoshStatusIcon, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(StatusIcon::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::info\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_info_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<StatusIcon>> StatusIconExt for O {}
