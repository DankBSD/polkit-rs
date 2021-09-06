// Generated by gir (https://github.com/gtk-rs/gir @ f64f90a)
// from /usr/local/share/gir-1.0 (@ ???)
// DO NOT EDIT

use crate::Subject;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    /// [`Permission`][crate::Permission] is a [`gio::Permission`][crate::gio::Permission] implementation. It can be used
    /// with e.g. `GtkLockButton`. See the [`gio::Permission`][crate::gio::Permission] documentation for
    /// more information.
    ///
    /// # Implements
    ///
    /// [`trait@gio::prelude::PermissionExt`], [`trait@glib::ObjectExt`]
    #[doc(alias = "PolkitPermission")]
    pub struct Permission(Object<ffi::PolkitPermission>) @extends gio::Permission;

    match fn {
        type_ => || ffi::polkit_permission_get_type(),
    }
}

impl Permission {
    /// Creates a [`gio::Permission`][crate::gio::Permission] instance for the PolicyKit action
    /// `action_id`.
    ///
    /// This is a synchronous failable constructor. See
    /// [`new()`][Self::new()] for the asynchronous version.
    /// ## `action_id`
    /// The PolicyKit action identifier.
    /// ## `subject`
    /// A [`Subject`][crate::Subject] or [`None`] for the current process.
    /// ## `cancellable`
    /// A [`gio::Cancellable`][crate::gio::Cancellable] or [`None`].
    ///
    /// # Returns
    ///
    /// A [`gio::Permission`][crate::gio::Permission] or [`None`] if `error` is set.
    #[doc(alias = "polkit_permission_new_sync")]
    pub fn new_sync<P: IsA<Subject>, Q: IsA<gio::Cancellable>>(
        action_id: &str,
        subject: Option<&P>,
        cancellable: Option<&Q>,
    ) -> Result<Permission, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::polkit_permission_new_sync(
                action_id.to_glib_none().0,
                subject.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(gio::Permission::from_glib_full(ret).unsafe_cast())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// Gets the PolicyKit action identifier used for `self`.
    ///
    /// # Returns
    ///
    /// A string owned by `self`. Do not free.
    #[doc(alias = "polkit_permission_get_action_id")]
    #[doc(alias = "get_action_id")]
    pub fn action_id(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::polkit_permission_get_action_id(self.to_glib_none().0)) }
    }

    /// Gets the subject used for `self`.
    ///
    /// # Returns
    ///
    /// An object owned by `self`. Do not free.
    #[doc(alias = "polkit_permission_get_subject")]
    #[doc(alias = "get_subject")]
    pub fn subject(&self) -> Subject {
        unsafe { from_glib_none(ffi::polkit_permission_get_subject(self.to_glib_none().0)) }
    }

    /// Creates a [`gio::Permission`][crate::gio::Permission] instance for the PolicyKit action
    /// `action_id`.
    ///
    /// When the operation is finished, `callback` will be invoked. You can
    /// then call `polkit_permission_new_finish()` to get the result of the
    /// operation.
    ///
    /// This is a asynchronous failable constructor. See
    /// [`new_sync()`][Self::new_sync()] for the synchronous version.
    /// ## `action_id`
    /// The PolicyKit action identifier.
    /// ## `subject`
    /// A [`Subject`][crate::Subject] or [`None`] for the current process.
    /// ## `cancellable`
    /// A [`gio::Cancellable`][crate::gio::Cancellable] or [`None`].
    /// ## `callback`
    /// A `GAsyncReadyCallback` to call when the request is satisfied.
    #[doc(alias = "polkit_permission_new")]
    pub fn new<
        P: IsA<Subject>,
        Q: IsA<gio::Cancellable>,
        R: FnOnce(Result<gio::Permission, glib::Error>) + Send + 'static,
    >(
        action_id: &str,
        subject: Option<&P>,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box_<R> = Box_::new(callback);
        unsafe extern "C" fn new_trampoline<
            R: FnOnce(Result<gio::Permission, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::polkit_permission_new_finish(res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<R> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = new_trampoline::<R>;
        unsafe {
            ffi::polkit_permission_new(
                action_id.to_glib_none().0,
                subject.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn new_future<P: IsA<Subject> + Clone + 'static>(
        action_id: &str,
        subject: Option<&P>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<gio::Permission, glib::Error>> + 'static>>
    {
        let action_id = String::from(action_id);
        let subject = subject.map(ToOwned::to_owned);
        Box_::pin(gio::GioFuture::new(&(), move |_obj, cancellable, send| {
            Self::new(
                &action_id,
                subject.as_ref().map(::std::borrow::Borrow::borrow),
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Permission")
    }
}
