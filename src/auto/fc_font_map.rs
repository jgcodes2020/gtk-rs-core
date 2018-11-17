// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use FontMap;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use pango;
use pango_ffi;
use std::fmt;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct FcFontMap(Object<ffi::PangoCairoFcFontMap>): [
        pango::FontMap => pango_ffi::PangoFontMap,
        FontMap,
    ];

    match fn {
        get_type => || ffi::pango_cairo_fc_font_map_get_type(),
    }
}

impl FcFontMap {}

impl fmt::Display for FcFontMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FcFontMap")
    }
}
