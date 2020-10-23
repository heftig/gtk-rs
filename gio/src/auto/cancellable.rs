// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Cancellable(Object<gio_sys::GCancellable, gio_sys::GCancellableClass, CancellableClass>);

    match fn {
        get_type => || gio_sys::g_cancellable_get_type(),
    }
}

impl Cancellable {
    pub fn new() -> Cancellable {
        unsafe { from_glib_full(gio_sys::g_cancellable_new()) }
    }

    pub fn get_current() -> Option<Cancellable> {
        unsafe { from_glib_none(gio_sys::g_cancellable_get_current()) }
    }
}

impl Default for Cancellable {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for Cancellable {}
unsafe impl Sync for Cancellable {}

pub const NONE_CANCELLABLE: Option<&Cancellable> = None;

pub trait CancellableExt: 'static {
    fn cancel(&self);

    //fn connect<P: Fn() + Send + Sync + 'static>(&self, callback: P, data: /*Unimplemented*/Option<Fundamental: Pointer>) -> libc::c_ulong;

    fn disconnect(&self, handler_id: libc::c_ulong);

    fn get_fd(&self) -> i32;

    fn is_cancelled(&self) -> bool;

    //fn make_pollfd(&self, pollfd: /*Ignored*/&mut glib::PollFD) -> bool;

    fn pop_current(&self);

    fn push_current(&self);

    fn release_fd(&self);

    fn set_error_if_cancelled(&self) -> Result<(), glib::Error>;

    fn connect_cancelled<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Cancellable>> CancellableExt for O {
    fn cancel(&self) {
        unsafe {
            gio_sys::g_cancellable_cancel(self.as_ref().to_glib_none().0);
        }
    }

    //fn connect<P: Fn() + Send + Sync + 'static>(&self, callback: P, data: /*Unimplemented*/Option<Fundamental: Pointer>) -> libc::c_ulong {
    //    unsafe { TODO: call gio_sys:g_cancellable_connect() }
    //}

    fn disconnect(&self, handler_id: libc::c_ulong) {
        unsafe {
            gio_sys::g_cancellable_disconnect(self.as_ref().to_glib_none().0, handler_id);
        }
    }

    fn get_fd(&self) -> i32 {
        unsafe { gio_sys::g_cancellable_get_fd(self.as_ref().to_glib_none().0) }
    }

    fn is_cancelled(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_cancellable_is_cancelled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn make_pollfd(&self, pollfd: /*Ignored*/&mut glib::PollFD) -> bool {
    //    unsafe { TODO: call gio_sys:g_cancellable_make_pollfd() }
    //}

    fn pop_current(&self) {
        unsafe {
            gio_sys::g_cancellable_pop_current(self.as_ref().to_glib_none().0);
        }
    }

    fn push_current(&self) {
        unsafe {
            gio_sys::g_cancellable_push_current(self.as_ref().to_glib_none().0);
        }
    }

    fn release_fd(&self) {
        unsafe {
            gio_sys::g_cancellable_release_fd(self.as_ref().to_glib_none().0);
        }
    }

    fn set_error_if_cancelled(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_cancellable_set_error_if_cancelled(
                self.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn connect_cancelled<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cancelled_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gio_sys::GCancellable,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Cancellable>,
        {
            let f: &F = &*(f as *const F);
            f(&Cancellable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cancelled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cancelled_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Cancellable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cancellable")
    }
}