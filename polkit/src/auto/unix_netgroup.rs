// Generated by gir (https://github.com/gtk-rs/gir @ 796942c)
// from /usr/local/share/gir-1.0 (@ ???)
// DO NOT EDIT

use crate::Identity;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    /// An object representing a netgroup identity on a UNIX system.
    ///
    /// # Implements
    ///
    /// [`trait@glib::ObjectExt`], [`IdentityExt`][trait@crate::prelude::IdentityExt]
    pub struct UnixNetgroup(Object<ffi::PolkitUnixNetgroup, ffi::PolkitUnixNetgroupClass>) @implements Identity;

    match fn {
        type_ => || ffi::polkit_unix_netgroup_get_type(),
    }
}

impl UnixNetgroup {
    /// Gets the netgroup name for `self`.
    ///
    /// # Returns
    ///
    /// A netgroup name string.
    #[doc(alias = "polkit_unix_netgroup_get_name")]
    #[doc(alias = "get_name")]
    pub fn name(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::polkit_unix_netgroup_get_name(self.to_glib_none().0)) }
    }

    /// Sets `name` for `self`.
    /// ## `name`
    /// A netgroup name.
    #[doc(alias = "polkit_unix_netgroup_set_name")]
    pub fn set_name(&self, name: &str) {
        unsafe {
            ffi::polkit_unix_netgroup_set_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    /// Creates a new [`UnixNetgroup`][crate::UnixNetgroup] object for `name`.
    /// ## `name`
    /// A netgroup name.
    ///
    /// # Returns
    ///
    /// A [`UnixNetgroup`][crate::UnixNetgroup] object. Free with `g_object_unref()`.
    #[doc(alias = "polkit_unix_netgroup_new")]
    pub fn new(name: &str) -> Identity {
        unsafe { from_glib_full(ffi::polkit_unix_netgroup_new(name.to_glib_none().0)) }
    }

    #[doc(alias = "name")]
    pub fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<F: Fn(&UnixNetgroup) + 'static>(
            this: *mut ffi::PolkitUnixNetgroup,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for UnixNetgroup {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.name())
    }
}
