// This file was generated by gir (c6a4ae6) from gir-files (11e0e6d)
// DO NOT EDIT

use Buildable;
#[cfg(gtk_3_6)]
use LevelBarMode;
use Orientable;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;
#[cfg(gtk_3_6)]
use std::mem;

glib_wrapper! {
    pub struct LevelBar(Object<ffi::GtkLevelBar>): Widget, Buildable, Orientable;

    match fn {
        get_type => || ffi::gtk_level_bar_get_type(),
    }
}

impl LevelBar {
    #[cfg(gtk_3_6)]
    pub fn new() -> LevelBar {
        unsafe {
            Widget::from_glib_none(ffi::gtk_level_bar_new()).downcast_unchecked()
        }
    }

    #[cfg(gtk_3_6)]
    pub fn new_for_interval(min_value: f64, max_value: f64) -> LevelBar {
        unsafe {
            Widget::from_glib_none(ffi::gtk_level_bar_new_for_interval(min_value, max_value)).downcast_unchecked()
        }
    }

    #[cfg(gtk_3_6)]
    pub fn add_offset_value(&self, name: &str, value: f64) {
        unsafe {
            ffi::gtk_level_bar_add_offset_value(self.to_glib_none().0, name.to_glib_none().0, value);
        }
    }

    #[cfg(gtk_3_8)]
    pub fn get_inverted(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_level_bar_get_inverted(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_6)]
    pub fn get_max_value(&self) -> f64 {
        unsafe {
            ffi::gtk_level_bar_get_max_value(self.to_glib_none().0)
        }
    }

    #[cfg(gtk_3_6)]
    pub fn get_min_value(&self) -> f64 {
        unsafe {
            ffi::gtk_level_bar_get_min_value(self.to_glib_none().0)
        }
    }

    #[cfg(gtk_3_6)]
    pub fn get_mode(&self) -> LevelBarMode {
        unsafe {
            ffi::gtk_level_bar_get_mode(self.to_glib_none().0)
        }
    }

    #[cfg(gtk_3_6)]
    pub fn get_offset_value(&self, name: Option<&str>) -> Option<f64> {
        unsafe {
            let mut value = mem::uninitialized();
            let ret = from_glib(ffi::gtk_level_bar_get_offset_value(self.to_glib_none().0, name.to_glib_none().0, &mut value));
            if ret { Some(value) } else { None }
        }
    }

    #[cfg(gtk_3_6)]
    pub fn get_value(&self) -> f64 {
        unsafe {
            ffi::gtk_level_bar_get_value(self.to_glib_none().0)
        }
    }

    #[cfg(gtk_3_6)]
    pub fn remove_offset_value(&self, name: Option<&str>) {
        unsafe {
            ffi::gtk_level_bar_remove_offset_value(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[cfg(gtk_3_8)]
    pub fn set_inverted(&self, inverted: bool) {
        unsafe {
            ffi::gtk_level_bar_set_inverted(self.to_glib_none().0, inverted.to_glib());
        }
    }

    #[cfg(gtk_3_6)]
    pub fn set_max_value(&self, value: f64) {
        unsafe {
            ffi::gtk_level_bar_set_max_value(self.to_glib_none().0, value);
        }
    }

    #[cfg(gtk_3_6)]
    pub fn set_min_value(&self, value: f64) {
        unsafe {
            ffi::gtk_level_bar_set_min_value(self.to_glib_none().0, value);
        }
    }

    #[cfg(gtk_3_6)]
    pub fn set_mode(&self, mode: LevelBarMode) {
        unsafe {
            ffi::gtk_level_bar_set_mode(self.to_glib_none().0, mode);
        }
    }

    #[cfg(gtk_3_6)]
    pub fn set_value(&self, value: f64) {
        unsafe {
            ffi::gtk_level_bar_set_value(self.to_glib_none().0, value);
        }
    }

}
