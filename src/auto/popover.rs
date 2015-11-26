// This file was generated by gir (c6a4ae6) from gir-files (11e0e6d)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use PositionType;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::Upcast;
use glib::translate::*;

glib_wrapper! {
    pub struct Popover(Object<ffi::GtkPopover>): Widget, Container, Bin, Buildable;

    match fn {
        get_type => || ffi::gtk_popover_get_type(),
    }
}

impl Popover {
    #[cfg(gtk_3_12)]
    pub fn new<T: Upcast<Widget>>(relative_to: Option<&T>) -> Popover {
        unsafe {
            Widget::from_glib_none(ffi::gtk_popover_new(relative_to.to_glib_none().0)).downcast_unchecked()
        }
    }

    //#[cfg(gtk_3_12)]
    //pub fn new_from_model<T: Upcast<Widget>, U: Upcast</*Ignored*/gio::MenuModel>>(relative_to: Option<&T>, model: &U) -> Popover {
    //    unsafe { TODO: call ffi::gtk_popover_new_from_model() }
    //}

}

pub trait PopoverExt {
    //#[cfg(gtk_3_12)]
    //fn bind_model<T: Upcast</*Ignored*/gio::MenuModel>>(&self, model: Option<&T>, action_namespace: Option<&str>);
    #[cfg(gtk_3_12)]
    fn get_modal(&self) -> bool;
    //fn get_pointing_to(&self, rect: /*Unknown conversion*/Unknown rust type: "Rectangle") -> bool;
    fn get_position(&self) -> PositionType;
    #[cfg(gtk_3_12)]
    fn get_relative_to(&self) -> Option<Widget>;
    #[cfg(gtk_3_16)]
    fn get_transitions_enabled(&self) -> bool;
    #[cfg(gtk_3_12)]
    fn set_modal(&self, modal: bool);
    //#[cfg(gtk_3_12)]
    //fn set_pointing_to(&self, rect: /*Unknown conversion*/Unknown rust type: "Rectangle");
    #[cfg(gtk_3_12)]
    fn set_position(&self, position: PositionType);
    #[cfg(gtk_3_12)]
    fn set_relative_to<T: Upcast<Widget>>(&self, relative_to: Option<&T>);
    #[cfg(gtk_3_16)]
    fn set_transitions_enabled(&self, transitions_enabled: bool);
}

impl<O: Upcast<Popover>> PopoverExt for O {
    //#[cfg(gtk_3_12)]
    //fn bind_model<T: Upcast</*Ignored*/gio::MenuModel>>(&self, model: Option<&T>, action_namespace: Option<&str>) {
    //    unsafe { TODO: call ffi::gtk_popover_bind_model() }
    //}

    #[cfg(gtk_3_12)]
    fn get_modal(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_popover_get_modal(self.to_glib_none().0))
        }
    }

    //fn get_pointing_to(&self, rect: /*Unknown conversion*/Unknown rust type: "Rectangle") -> bool {
    //    unsafe { TODO: call ffi::gtk_popover_get_pointing_to() }
    //}

    fn get_position(&self) -> PositionType {
        unsafe {
            ffi::gtk_popover_get_position(self.to_glib_none().0)
        }
    }

    #[cfg(gtk_3_12)]
    fn get_relative_to(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_popover_get_relative_to(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_16)]
    fn get_transitions_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_popover_get_transitions_enabled(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_12)]
    fn set_modal(&self, modal: bool) {
        unsafe {
            ffi::gtk_popover_set_modal(self.to_glib_none().0, modal.to_glib());
        }
    }

    //#[cfg(gtk_3_12)]
    //fn set_pointing_to(&self, rect: /*Unknown conversion*/Unknown rust type: "Rectangle") {
    //    unsafe { TODO: call ffi::gtk_popover_set_pointing_to() }
    //}

    #[cfg(gtk_3_12)]
    fn set_position(&self, position: PositionType) {
        unsafe {
            ffi::gtk_popover_set_position(self.to_glib_none().0, position);
        }
    }

    #[cfg(gtk_3_12)]
    fn set_relative_to<T: Upcast<Widget>>(&self, relative_to: Option<&T>) {
        unsafe {
            ffi::gtk_popover_set_relative_to(self.to_glib_none().0, relative_to.to_glib_none().0);
        }
    }

    #[cfg(gtk_3_16)]
    fn set_transitions_enabled(&self, transitions_enabled: bool) {
        unsafe {
            ffi::gtk_popover_set_transitions_enabled(self.to_glib_none().0, transitions_enabled.to_glib());
        }
    }

}
