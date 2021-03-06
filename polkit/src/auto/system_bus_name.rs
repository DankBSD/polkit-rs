// Generated by gir (https://github.com/gtk-rs/gir @ 796942c)
// from /usr/local/share/gir-1.0 (@ ???)
// DO NOT EDIT

use crate::Subject;
use crate::UnixUser;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    /// An object that represents a process owning a unique name on the system bus.
    ///
    /// # Implements
    ///
    /// [`trait@glib::ObjectExt`], [`SubjectExt`][trait@crate::prelude::SubjectExt]
    pub struct SystemBusName(Object<ffi::PolkitSystemBusName, ffi::PolkitSystemBusNameClass>) @implements Subject;

    match fn {
        type_ => || ffi::polkit_system_bus_name_get_type(),
    }
}

impl SystemBusName {
    /// Gets the unique system bus name for `self`.
    ///
    /// # Returns
    ///
    /// The unique system bus name for `self`. Do not
    /// free, this string is owned by `self`.
    #[doc(alias = "polkit_system_bus_name_get_name")]
    #[doc(alias = "get_name")]
    pub fn name(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::polkit_system_bus_name_get_name(self.to_glib_none().0)) }
    }

    /// Synchronously gets a [`UnixProcess`][crate::UnixProcess] object for `self`
    /// - the calling thread is blocked until a reply is received.
    /// ## `cancellable`
    /// A [`gio::Cancellable`][crate::gio::Cancellable] or [`None`].
    ///
    /// # Returns
    ///
    /// A [`UnixProcess`][crate::UnixProcess] object or [`None`] if `error` is set.
    #[doc(alias = "polkit_system_bus_name_get_process_sync")]
    #[doc(alias = "get_process_sync")]
    pub fn process_sync<P: IsA<gio::Cancellable>>(
        &self,
        cancellable: Option<&P>,
    ) -> Result<Option<Subject>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::polkit_system_bus_name_get_process_sync(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// Synchronously gets a [`UnixUser`][crate::UnixUser] object for `self`;
    /// the calling thread is blocked until a reply is received.
    /// ## `cancellable`
    /// A [`gio::Cancellable`][crate::gio::Cancellable] or [`None`].
    ///
    /// # Returns
    ///
    /// A [`UnixUser`][crate::UnixUser] object or [`None`] if `error` is set.
    #[doc(alias = "polkit_system_bus_name_get_user_sync")]
    #[doc(alias = "get_user_sync")]
    pub fn user_sync<P: IsA<gio::Cancellable>>(
        &self,
        cancellable: Option<&P>,
    ) -> Result<Option<UnixUser>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::polkit_system_bus_name_get_user_sync(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// Sets the unique system bus name for `self`.
    /// ## `name`
    /// A unique system bus name.
    #[doc(alias = "polkit_system_bus_name_set_name")]
    pub fn set_name(&self, name: &str) {
        unsafe {
            ffi::polkit_system_bus_name_set_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    /// Creates a new [`SystemBusName`][crate::SystemBusName] for `name`.
    /// ## `name`
    /// A unique system bus name.
    ///
    /// # Returns
    ///
    /// A [`SystemBusName`][crate::SystemBusName]. Free with `g_object_unref()`.
    #[doc(alias = "polkit_system_bus_name_new")]
    pub fn new(name: &str) -> Subject {
        unsafe { from_glib_full(ffi::polkit_system_bus_name_new(name.to_glib_none().0)) }
    }

    #[doc(alias = "name")]
    pub fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<F: Fn(&SystemBusName) + 'static>(
            this: *mut ffi::PolkitSystemBusName,
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

impl fmt::Display for SystemBusName {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.name())
    }
}
