// This file was generated by gir (94e079d) from gir-files (469db10)
// DO NOT EDIT

use ChecksumType;
use ffi;
use ffi as glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;
use translate::*;

glib_wrapper! {
    pub struct Checksum(Boxed<ffi::GChecksum>);

    match fn {
        copy => |ptr| ffi::g_checksum_copy(mut_override(ptr)),
        free => |ptr| ffi::g_checksum_free(ptr),
        get_type => || ffi::g_checksum_get_type(),
    }
}

impl Checksum {
    pub fn new(checksum_type: ChecksumType) -> Checksum {
        unsafe {
            from_glib_full(ffi::g_checksum_new(checksum_type.to_glib()))
        }
    }

    pub fn reset(&mut self) {
        unsafe {
            ffi::g_checksum_reset(self.to_glib_none_mut().0);
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        let length = data.len() as isize;
        unsafe {
            ffi::g_checksum_update(self.to_glib_none_mut().0, data.to_glib_none().0, length);
        }
    }

    pub fn type_get_length(checksum_type: ChecksumType) -> isize {
        unsafe {
            ffi::g_checksum_type_get_length(checksum_type.to_glib())
        }
    }
}
