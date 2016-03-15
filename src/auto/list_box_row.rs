// This file was generated by gir (0d8699e) from gir-files (11e0e6d)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct ListBoxRow(Object<ffi::GtkListBoxRow>): Bin, Container, Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_list_box_row_get_type(),
    }
}

impl ListBoxRow {
    #[cfg(feature = "v3_10")]
    pub fn new() -> ListBoxRow {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_list_box_row_new()).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn changed(&self) {
        unsafe {
            ffi::gtk_list_box_row_changed(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn get_activatable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_box_row_get_activatable(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_header(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_row_get_header(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_index(&self) -> i32 {
        unsafe {
            ffi::gtk_list_box_row_get_index(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn get_selectable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_box_row_get_selectable(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn is_selected(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_box_row_is_selected(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn set_activatable(&self, activatable: bool) {
        unsafe {
            ffi::gtk_list_box_row_set_activatable(self.to_glib_none().0, activatable.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn set_header<T: IsA<Widget>>(&self, header: Option<&T>) {
        unsafe {
            ffi::gtk_list_box_row_set_header(self.to_glib_none().0, header.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn set_selectable(&self, selectable: bool) {
        unsafe {
            ffi::gtk_list_box_row_set_selectable(self.to_glib_none().0, selectable.to_glib());
        }
    }
}
