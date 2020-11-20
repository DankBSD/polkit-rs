// This file was generated by gir (https://github.com/gtk-rs/gir @ cea2f7c)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use glib;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use polkit_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;
use Identity;

glib_wrapper! {
    pub struct UnixGroup(Object<polkit_sys::PolkitUnixGroup, polkit_sys::PolkitUnixGroupClass, UnixGroupClass>) @implements Identity;

    match fn {
        get_type => || polkit_sys::polkit_unix_group_get_type(),
    }
}

impl UnixGroup {
    /// Gets the UNIX group id for `self`.
    ///
    /// # Returns
    ///
    /// A UNIX group id.
    pub fn get_gid(&self) -> i32 {
        unsafe {
            polkit_sys::polkit_unix_group_get_gid(self.to_glib_none().0)
        }
    }

    /// Sets `gid` for `self`.
    /// ## `gid`
    /// A UNIX group id.
    pub fn set_gid(&self, gid: i32) {
        unsafe {
            polkit_sys::polkit_unix_group_set_gid(self.to_glib_none().0, gid);
        }
    }

    /// Creates a new `UnixGroup` object for `gid`.
    /// ## `gid`
    /// A UNIX group id.
    ///
    /// # Returns
    ///
    /// A `UnixGroup` object. Free with `gobject::ObjectExt::unref`.
    pub fn new(gid: i32) -> Identity {
        unsafe {
            from_glib_full(polkit_sys::polkit_unix_group_new(gid))
        }
    }

    /// Creates a new `UnixGroup` object for a group with the group name
    /// `name`.
    /// ## `name`
    /// A UNIX group name.
    ///
    /// # Returns
    ///
    /// (allow-none): A `UnixGroup` object or `None` if `error`
    /// is set.
    pub fn new_for_name(name: &str) -> Result<Identity, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = polkit_sys::polkit_unix_group_new_for_name(name.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn connect_property_gid_notify<F: Fn(&UnixGroup) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_gid_trampoline<F: Fn(&UnixGroup) + 'static>(this: *mut polkit_sys::PolkitUnixGroup, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::gid\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_gid_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for UnixGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UnixGroup")
    }
}
