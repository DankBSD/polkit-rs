// Generated by gir (https://github.com/gtk-rs/gir @ 796942c)
// from /usr/local/share/gir-1.0 (@ ???)
// DO NOT EDIT

use crate::Subject;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    /// An object that represents an user session.
    ///
    /// The session id is an opaque string obtained from ConsoleKit.
    ///
    /// # Implements
    ///
    /// [`trait@glib::ObjectExt`], [`SubjectExt`][trait@crate::prelude::SubjectExt]
    pub struct UnixSession(Object<ffi::PolkitUnixSession, ffi::PolkitUnixSessionClass>) @implements Subject;

    match fn {
        type_ => || ffi::polkit_unix_session_get_type(),
    }
}

impl UnixSession {
    /// Gets the session id for `self`.
    ///
    /// # Returns
    ///
    /// The session id for `self`. Do not free this string, it
    /// is owned by `self`.
    #[doc(alias = "polkit_unix_session_get_session_id")]
    #[doc(alias = "get_session_id")]
    pub fn session_id(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::polkit_unix_session_get_session_id(
                self.to_glib_none().0,
            ))
        }
    }

    /// Sets the session id for `self` to `session_id`.
    /// ## `session_id`
    /// The session id.
    #[doc(alias = "polkit_unix_session_set_session_id")]
    pub fn set_session_id(&self, session_id: &str) {
        unsafe {
            ffi::polkit_unix_session_set_session_id(
                self.to_glib_none().0,
                session_id.to_glib_none().0,
            );
        }
    }

    /// Creates a new [`UnixSession`][crate::UnixSession] for `session_id`.
    /// ## `session_id`
    /// The session id.
    ///
    /// # Returns
    ///
    /// A [`UnixSession`][crate::UnixSession]. Free with `g_object_unref()`.
    #[doc(alias = "polkit_unix_session_new")]
    pub fn new(session_id: &str) -> Subject {
        unsafe { from_glib_full(ffi::polkit_unix_session_new(session_id.to_glib_none().0)) }
    }

    /// Asynchronously creates a new [`UnixSession`][crate::UnixSession] object for the
    /// process with process id `pid`.
    ///
    /// When the operation is finished, `callback` will be invoked in the
    /// <link linkend="g-main-context-push-thread-default">thread-default
    /// main loop`</link>` of the thread you are calling this method
    /// from. You can then call
    /// `polkit_unix_session_new_for_process_finish()` to get the result of
    /// the operation.
    ///
    /// This method constructs the object asynchronously, for the synchronous and blocking version
    /// use [`new_for_process_sync()`][Self::new_for_process_sync()].
    /// ## `pid`
    /// The process id of the process to get the session for.
    /// ## `cancellable`
    /// A [`gio::Cancellable`][crate::gio::Cancellable] or [`None`].
    /// ## `callback`
    /// A `GAsyncReadyCallback` to call when the request is satisfied
    #[doc(alias = "polkit_unix_session_new_for_process")]
    pub fn new_for_process<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<Option<Subject>, glib::Error>) + Send + 'static,
    >(
        pid: i32,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn new_for_process_trampoline<
            Q: FnOnce(Result<Option<Subject>, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::polkit_unix_session_new_for_process_finish(res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = new_for_process_trampoline::<Q>;
        unsafe {
            ffi::polkit_unix_session_new_for_process(
                pid,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn new_for_process_future(
        pid: i32,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Option<Subject>, glib::Error>> + 'static>>
    {
        Box_::pin(gio::GioFuture::new(&(), move |_obj, send| {
            let cancellable = gio::Cancellable::new();
            Self::new_for_process(pid, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    /// Creates a new [`UnixSession`][crate::UnixSession] for the process with process id `pid`.
    ///
    /// This is a synchronous call - the calling thread is blocked until a
    /// reply is received. For the asynchronous version, see
    /// [`new_for_process()`][Self::new_for_process()].
    /// ## `pid`
    /// The process id of the process to get the session for.
    /// ## `cancellable`
    /// A [`gio::Cancellable`][crate::gio::Cancellable] or [`None`].
    ///
    /// # Returns
    ///
    /// A [`UnixSession`][crate::UnixSession] for
    /// `pid` or [`None`] if `error` is set. Free with `g_object_unref()`.
    #[doc(alias = "polkit_unix_session_new_for_process_sync")]
    pub fn new_for_process_sync<P: IsA<gio::Cancellable>>(
        pid: i32,
        cancellable: Option<&P>,
    ) -> Result<Option<Subject>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::polkit_unix_session_new_for_process_sync(
                pid,
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

    #[doc(alias = "session-id")]
    pub fn connect_session_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_session_id_trampoline<F: Fn(&UnixSession) + 'static>(
            this: *mut ffi::PolkitUnixSession,
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
                b"notify::session-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_session_id_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for UnixSession {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("UnixSession")
    }
}
