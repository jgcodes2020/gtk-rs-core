// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use gdk_pixbuf;
use gio;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::value::SetValueOptional;
use glib::GString;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use CellRenderer;
use CellRendererMode;

glib_wrapper! {
    pub struct CellRendererPixbuf(Object<gtk_sys::GtkCellRendererPixbuf, gtk_sys::GtkCellRendererPixbufClass, CellRendererPixbufClass>) @extends CellRenderer;

    match fn {
        get_type => || gtk_sys::gtk_cell_renderer_pixbuf_get_type(),
    }
}

impl CellRendererPixbuf {
    pub fn new() -> CellRendererPixbuf {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(gtk_sys::gtk_cell_renderer_pixbuf_new()).unsafe_cast()
        }
    }
}

impl Default for CellRendererPixbuf {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CellRendererPixbufBuilder {
    follow_state: Option<bool>,
    gicon: Option<gio::Icon>,
    icon_name: Option<String>,
    pixbuf: Option<gdk_pixbuf::Pixbuf>,
    pixbuf_expander_closed: Option<gdk_pixbuf::Pixbuf>,
    pixbuf_expander_open: Option<gdk_pixbuf::Pixbuf>,
    stock_detail: Option<String>,
    stock_size: Option<u32>,
    cell_background: Option<String>,
    cell_background_rgba: Option<gdk::RGBA>,
    cell_background_set: Option<bool>,
    height: Option<i32>,
    is_expanded: Option<bool>,
    is_expander: Option<bool>,
    mode: Option<CellRendererMode>,
    sensitive: Option<bool>,
    visible: Option<bool>,
    width: Option<i32>,
    xalign: Option<f32>,
    xpad: Option<u32>,
    yalign: Option<f32>,
    ypad: Option<u32>,
}

impl CellRendererPixbufBuilder {
    pub fn new() -> Self {
        Self {
            follow_state: None,
            gicon: None,
            icon_name: None,
            pixbuf: None,
            pixbuf_expander_closed: None,
            pixbuf_expander_open: None,
            stock_detail: None,
            stock_size: None,
            cell_background: None,
            cell_background_rgba: None,
            cell_background_set: None,
            height: None,
            is_expanded: None,
            is_expander: None,
            mode: None,
            sensitive: None,
            visible: None,
            width: None,
            xalign: None,
            xpad: None,
            yalign: None,
            ypad: None,
        }
    }

    pub fn build(self) -> CellRendererPixbuf {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref follow_state) = self.follow_state {
            properties.push(("follow-state", follow_state));
        }
        if let Some(ref gicon) = self.gicon {
            properties.push(("gicon", gicon));
        }
        if let Some(ref icon_name) = self.icon_name {
            properties.push(("icon-name", icon_name));
        }
        if let Some(ref pixbuf) = self.pixbuf {
            properties.push(("pixbuf", pixbuf));
        }
        if let Some(ref pixbuf_expander_closed) = self.pixbuf_expander_closed {
            properties.push(("pixbuf-expander-closed", pixbuf_expander_closed));
        }
        if let Some(ref pixbuf_expander_open) = self.pixbuf_expander_open {
            properties.push(("pixbuf-expander-open", pixbuf_expander_open));
        }
        if let Some(ref stock_detail) = self.stock_detail {
            properties.push(("stock-detail", stock_detail));
        }
        if let Some(ref stock_size) = self.stock_size {
            properties.push(("stock-size", stock_size));
        }
        if let Some(ref cell_background) = self.cell_background {
            properties.push(("cell-background", cell_background));
        }
        if let Some(ref cell_background_rgba) = self.cell_background_rgba {
            properties.push(("cell-background-rgba", cell_background_rgba));
        }
        if let Some(ref cell_background_set) = self.cell_background_set {
            properties.push(("cell-background-set", cell_background_set));
        }
        if let Some(ref height) = self.height {
            properties.push(("height", height));
        }
        if let Some(ref is_expanded) = self.is_expanded {
            properties.push(("is-expanded", is_expanded));
        }
        if let Some(ref is_expander) = self.is_expander {
            properties.push(("is-expander", is_expander));
        }
        if let Some(ref mode) = self.mode {
            properties.push(("mode", mode));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width) = self.width {
            properties.push(("width", width));
        }
        if let Some(ref xalign) = self.xalign {
            properties.push(("xalign", xalign));
        }
        if let Some(ref xpad) = self.xpad {
            properties.push(("xpad", xpad));
        }
        if let Some(ref yalign) = self.yalign {
            properties.push(("yalign", yalign));
        }
        if let Some(ref ypad) = self.ypad {
            properties.push(("ypad", ypad));
        }
        glib::Object::new(CellRendererPixbuf::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn follow_state(mut self, follow_state: bool) -> Self {
        self.follow_state = Some(follow_state);
        self
    }

    pub fn gicon(mut self, gicon: &gio::Icon) -> Self {
        self.gicon = Some(gicon.clone());
        self
    }

    pub fn icon_name(mut self, icon_name: &str) -> Self {
        self.icon_name = Some(icon_name.to_string());
        self
    }

    pub fn pixbuf(mut self, pixbuf: &gdk_pixbuf::Pixbuf) -> Self {
        self.pixbuf = Some(pixbuf.clone());
        self
    }

    pub fn pixbuf_expander_closed(mut self, pixbuf_expander_closed: &gdk_pixbuf::Pixbuf) -> Self {
        self.pixbuf_expander_closed = Some(pixbuf_expander_closed.clone());
        self
    }

    pub fn pixbuf_expander_open(mut self, pixbuf_expander_open: &gdk_pixbuf::Pixbuf) -> Self {
        self.pixbuf_expander_open = Some(pixbuf_expander_open.clone());
        self
    }

    pub fn stock_detail(mut self, stock_detail: &str) -> Self {
        self.stock_detail = Some(stock_detail.to_string());
        self
    }

    pub fn stock_size(mut self, stock_size: u32) -> Self {
        self.stock_size = Some(stock_size);
        self
    }

    pub fn cell_background(mut self, cell_background: &str) -> Self {
        self.cell_background = Some(cell_background.to_string());
        self
    }

    pub fn cell_background_rgba(mut self, cell_background_rgba: &gdk::RGBA) -> Self {
        self.cell_background_rgba = Some(cell_background_rgba.clone());
        self
    }

    pub fn cell_background_set(mut self, cell_background_set: bool) -> Self {
        self.cell_background_set = Some(cell_background_set);
        self
    }

    pub fn height(mut self, height: i32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn is_expanded(mut self, is_expanded: bool) -> Self {
        self.is_expanded = Some(is_expanded);
        self
    }

    pub fn is_expander(mut self, is_expander: bool) -> Self {
        self.is_expander = Some(is_expander);
        self
    }

    pub fn mode(mut self, mode: CellRendererMode) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width(mut self, width: i32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn xalign(mut self, xalign: f32) -> Self {
        self.xalign = Some(xalign);
        self
    }

    pub fn xpad(mut self, xpad: u32) -> Self {
        self.xpad = Some(xpad);
        self
    }

    pub fn yalign(mut self, yalign: f32) -> Self {
        self.yalign = Some(yalign);
        self
    }

    pub fn ypad(mut self, ypad: u32) -> Self {
        self.ypad = Some(ypad);
        self
    }
}

pub const NONE_CELL_RENDERER_PIXBUF: Option<&CellRendererPixbuf> = None;

pub trait CellRendererPixbufExt: 'static {
    #[cfg_attr(feature = "v3_16", deprecated)]
    fn get_property_follow_state(&self) -> bool;

    #[cfg_attr(feature = "v3_16", deprecated)]
    fn set_property_follow_state(&self, follow_state: bool);

    fn get_property_gicon(&self) -> Option<gio::Icon>;

    fn set_property_gicon<P: IsA<gio::Icon> + SetValueOptional>(&self, gicon: Option<&P>);

    fn get_property_icon_name(&self) -> Option<GString>;

    fn set_property_icon_name(&self, icon_name: Option<&str>);

    fn get_property_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf>;

    fn set_property_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>);

    fn get_property_pixbuf_expander_closed(&self) -> Option<gdk_pixbuf::Pixbuf>;

    fn set_property_pixbuf_expander_closed(
        &self,
        pixbuf_expander_closed: Option<&gdk_pixbuf::Pixbuf>,
    );

    fn get_property_pixbuf_expander_open(&self) -> Option<gdk_pixbuf::Pixbuf>;

    fn set_property_pixbuf_expander_open(&self, pixbuf_expander_open: Option<&gdk_pixbuf::Pixbuf>);

    fn get_property_stock_detail(&self) -> Option<GString>;

    fn set_property_stock_detail(&self, stock_detail: Option<&str>);

    #[cfg_attr(feature = "v3_16", deprecated)]
    fn connect_property_follow_state_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pixbuf_expander_closed_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_pixbuf_expander_open_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_stock_detail_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_stock_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CellRendererPixbuf>> CellRendererPixbufExt for O {
    fn get_property_follow_state(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"follow-state\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `follow-state` getter")
                .unwrap()
        }
    }

    fn set_property_follow_state(&self, follow_state: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"follow-state\0".as_ptr() as *const _,
                Value::from(&follow_state).to_glib_none().0,
            );
        }
    }

    fn get_property_gicon(&self) -> Option<gio::Icon> {
        unsafe {
            let mut value = Value::from_type(<gio::Icon as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"gicon\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `gicon` getter")
        }
    }

    fn set_property_gicon<P: IsA<gio::Icon> + SetValueOptional>(&self, gicon: Option<&P>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"gicon\0".as_ptr() as *const _,
                Value::from(gicon).to_glib_none().0,
            );
        }
    }

    fn get_property_icon_name(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"icon-name\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `icon-name` getter")
        }
    }

    fn set_property_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"icon-name\0".as_ptr() as *const _,
                Value::from(icon_name).to_glib_none().0,
            );
        }
    }

    fn get_property_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            let mut value = Value::from_type(<gdk_pixbuf::Pixbuf as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"pixbuf\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `pixbuf` getter")
        }
    }

    fn set_property_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"pixbuf\0".as_ptr() as *const _,
                Value::from(pixbuf).to_glib_none().0,
            );
        }
    }

    fn get_property_pixbuf_expander_closed(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            let mut value = Value::from_type(<gdk_pixbuf::Pixbuf as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"pixbuf-expander-closed\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `pixbuf-expander-closed` getter")
        }
    }

    fn set_property_pixbuf_expander_closed(
        &self,
        pixbuf_expander_closed: Option<&gdk_pixbuf::Pixbuf>,
    ) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"pixbuf-expander-closed\0".as_ptr() as *const _,
                Value::from(pixbuf_expander_closed).to_glib_none().0,
            );
        }
    }

    fn get_property_pixbuf_expander_open(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            let mut value = Value::from_type(<gdk_pixbuf::Pixbuf as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"pixbuf-expander-open\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `pixbuf-expander-open` getter")
        }
    }

    fn set_property_pixbuf_expander_open(&self, pixbuf_expander_open: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"pixbuf-expander-open\0".as_ptr() as *const _,
                Value::from(pixbuf_expander_open).to_glib_none().0,
            );
        }
    }

    fn get_property_stock_detail(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"stock-detail\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `stock-detail` getter")
        }
    }

    fn set_property_stock_detail(&self, stock_detail: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"stock-detail\0".as_ptr() as *const _,
                Value::from(stock_detail).to_glib_none().0,
            );
        }
    }

    fn connect_property_follow_state_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_follow_state_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkCellRendererPixbuf,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<CellRendererPixbuf>,
        {
            let f: &F = &*(f as *const F);
            f(&CellRendererPixbuf::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::follow-state\0".as_ptr() as *const _,
                Some(transmute(
                    notify_follow_state_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_gicon_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkCellRendererPixbuf,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<CellRendererPixbuf>,
        {
            let f: &F = &*(f as *const F);
            f(&CellRendererPixbuf::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::gicon\0".as_ptr() as *const _,
                Some(transmute(notify_gicon_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkCellRendererPixbuf,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<CellRendererPixbuf>,
        {
            let f: &F = &*(f as *const F);
            f(&CellRendererPixbuf::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icon-name\0".as_ptr() as *const _,
                Some(transmute(notify_icon_name_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pixbuf_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkCellRendererPixbuf,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<CellRendererPixbuf>,
        {
            let f: &F = &*(f as *const F);
            f(&CellRendererPixbuf::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pixbuf\0".as_ptr() as *const _,
                Some(transmute(notify_pixbuf_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_pixbuf_expander_closed_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_pixbuf_expander_closed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkCellRendererPixbuf,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<CellRendererPixbuf>,
        {
            let f: &F = &*(f as *const F);
            f(&CellRendererPixbuf::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pixbuf-expander-closed\0".as_ptr() as *const _,
                Some(transmute(
                    notify_pixbuf_expander_closed_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_pixbuf_expander_open_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_pixbuf_expander_open_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkCellRendererPixbuf,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<CellRendererPixbuf>,
        {
            let f: &F = &*(f as *const F);
            f(&CellRendererPixbuf::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pixbuf-expander-open\0".as_ptr() as *const _,
                Some(transmute(
                    notify_pixbuf_expander_open_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_stock_detail_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_stock_detail_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkCellRendererPixbuf,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<CellRendererPixbuf>,
        {
            let f: &F = &*(f as *const F);
            f(&CellRendererPixbuf::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::stock-detail\0".as_ptr() as *const _,
                Some(transmute(
                    notify_stock_detail_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_stock_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_stock_size_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkCellRendererPixbuf,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<CellRendererPixbuf>,
        {
            let f: &F = &*(f as *const F);
            f(&CellRendererPixbuf::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::stock-size\0".as_ptr() as *const _,
                Some(transmute(notify_stock_size_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for CellRendererPixbuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CellRendererPixbuf")
    }
}
