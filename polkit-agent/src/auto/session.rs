// Generated by gir (https://github.com/gtk-rs/gir @ 796942c)
// from /usr/local/share/gir-1.0 (@ ???)
// DO NOT EDIT

use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    /// The [`Session`][crate::Session] class is an abstraction used for interacting with the
    /// native authentication system (for example PAM) for obtaining authorizations.
    /// This class is typically used together with instances that are derived from
    /// the `PolkitAgentListener` abstract base class.
    ///
    /// To perform the actual authentication, [`Session`][crate::Session] uses a trusted suid helper.
    /// The authentication conversation is done through a pipe. This is transparent; the user
    /// only need to handle the
    /// `signal::Session::request`,
    /// `signal::Session::show-info`,
    /// `signal::Session::show-error` and
    /// `signal::Session::completed`
    /// signals and invoke [`response()`][Self::response()] in response to requests.
    ///
    /// If the user successfully authenticates, the authentication helper will invoke
    /// a method on the PolicyKit daemon (see `polkit_authority_authentication_agent_response_sync()`)
    /// with the given `cookie`. Upon receiving a positive response from the PolicyKit daemon (via
    /// the authentication helper), the `signal::Session::completed` signal will be emitted
    /// with the `gained_authorization` paramter set to [`true`].
    ///
    /// If the user is unable to authenticate, the `signal::Session::completed` signal will
    /// be emitted with the `gained_authorization` paramter set to [`false`].
    ///
    /// # Implements
    ///
    /// [`trait@glib::ObjectExt`]
    pub struct Session(Object<ffi::PolkitAgentSession, ffi::PolkitAgentSessionClass>);

    match fn {
        type_ => || ffi::polkit_agent_session_get_type(),
    }
}

impl Session {
    /// Creates a new authentication session.
    ///
    /// The caller should connect to the
    /// `signal::Session::request`,
    /// `signal::Session::show-info`,
    /// `signal::Session::show-error` and
    /// `signal::Session::completed`
    /// signals and then call [`initiate()`][Self::initiate()] to initiate the authentication session.
    /// ## `identity`
    /// The identity to authenticate.
    /// ## `cookie`
    /// The cookie obtained from the PolicyKit daemon
    ///
    /// # Returns
    ///
    /// A [`Session`][crate::Session]. Free with `g_object_unref()`.
    #[doc(alias = "polkit_agent_session_new")]
    pub fn new<P: IsA<polkit::Identity>>(identity: &P, cookie: &str) -> Session {
        unsafe {
            from_glib_full(ffi::polkit_agent_session_new(
                identity.as_ref().to_glib_none().0,
                cookie.to_glib_none().0,
            ))
        }
    }

    /// Cancels an authentication session. This will make `self` emit the `signal::Session::completed`
    /// signal.
    #[doc(alias = "polkit_agent_session_cancel")]
    pub fn cancel(&self) {
        unsafe {
            ffi::polkit_agent_session_cancel(self.to_glib_none().0);
        }
    }

    /// Initiates the authentication session. Before calling this method,
    /// make sure to connect to the various signals. The signals will be
    /// emitted in the <link
    /// linkend="g-main-context-push-thread-default">thread-default main
    /// loop`</link>` that this method is invoked from.
    ///
    /// Use [`cancel()`][Self::cancel()] to cancel the session.
    #[doc(alias = "polkit_agent_session_initiate")]
    pub fn initiate(&self) {
        unsafe {
            ffi::polkit_agent_session_initiate(self.to_glib_none().0);
        }
    }

    /// Function for providing response to requests received
    /// via the `signal::Session::request` signal.
    /// ## `response`
    /// Response from the user, typically a password.
    #[doc(alias = "polkit_agent_session_response")]
    pub fn response(&self, response: &str) {
        unsafe {
            ffi::polkit_agent_session_response(self.to_glib_none().0, response.to_glib_none().0);
        }
    }

    /// The cookie obtained from the PolicyKit daemon
    pub fn cookie(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"cookie\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `cookie` getter")
        }
    }

    /// The identity to authenticate.
    pub fn identity(&self) -> Option<polkit::Identity> {
        unsafe {
            let mut value = glib::Value::from_type(<polkit::Identity as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"identity\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `identity` getter")
        }
    }

    /// Emitted when the authentication session has been completed or
    /// cancelled. The `gained_authorization` parameter is [`true`] only if
    /// the user successfully authenticated.
    ///
    /// Upon receiving this signal, the user should free `session` using `g_object_unref()`.
    /// ## `gained_authorization`
    /// [`true`] only if the authorization was successfully obtained.
    #[doc(alias = "completed")]
    pub fn connect_completed<F: Fn(&Self, bool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn completed_trampoline<F: Fn(&Session, bool) + 'static>(
            this: *mut ffi::PolkitAgentSession,
            gained_authorization: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(gained_authorization))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"completed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    completed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    /// Emitted when the user is requested to answer a question.
    ///
    /// When the response has been collected from the user, call [`response()`][Self::response()].
    /// ## `request`
    /// The request to show the user, e.g. "name: " or "password: ".
    /// ## `echo_on`
    /// [`true`] if the response to the request SHOULD be echoed on the
    ///  screen, [`false`] if the response MUST NOT be echoed to the screen.
    #[doc(alias = "request")]
    pub fn connect_request<F: Fn(&Self, &str, bool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn request_trampoline<F: Fn(&Session, &str, bool) + 'static>(
            this: *mut ffi::PolkitAgentSession,
            request: *mut libc::c_char,
            echo_on: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &glib::GString::from_glib_borrow(request),
                from_glib(echo_on),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"request\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    request_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    /// Emitted when there is information related to an error condition to be displayed to the user.
    /// ## `text`
    /// An error string to display to the user.
    #[doc(alias = "show-error")]
    pub fn connect_show_error<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn show_error_trampoline<F: Fn(&Session, &str) + 'static>(
            this: *mut ffi::PolkitAgentSession,
            text: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &glib::GString::from_glib_borrow(text),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"show-error\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    show_error_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    /// Emitted when there is information to be displayed to the user.
    /// ## `text`
    /// A string to display to the user.
    #[doc(alias = "show-info")]
    pub fn connect_show_info<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn show_info_trampoline<F: Fn(&Session, &str) + 'static>(
            this: *mut ffi::PolkitAgentSession,
            text: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &glib::GString::from_glib_borrow(text),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"show-info\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    show_info_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Session {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Session")
    }
}
