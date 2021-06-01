// Generated by gir (https://github.com/gtk-rs/gir @ 796942c)
// from /usr/local/share/gir-1.0 (@ ???)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal,
    clippy::upper_case_acronyms
)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

use gio_sys as gio;
use glib_sys as glib;
use gobject_sys as gobject;
use polkit_sys as polkit;

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, time_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Flags
pub type PolkitAgentRegisterFlags = c_uint;
pub const POLKIT_AGENT_REGISTER_FLAGS_NONE: PolkitAgentRegisterFlags = 0;
pub const POLKIT_AGENT_REGISTER_FLAGS_RUN_IN_THREAD: PolkitAgentRegisterFlags = 1;

// Records
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PolkitAgentListenerClass {
    pub parent_class: gobject::GObjectClass,
    pub initiate_authentication: Option<
        unsafe extern "C" fn(
            *mut PolkitAgentListener,
            *const c_char,
            *const c_char,
            *const c_char,
            *mut polkit::PolkitDetails,
            *const c_char,
            *mut glib::GList,
            *mut gio::GCancellable,
            gio::GAsyncReadyCallback,
            gpointer,
        ),
    >,
    pub initiate_authentication_finish: Option<
        unsafe extern "C" fn(
            *mut PolkitAgentListener,
            *mut gio::GAsyncResult,
            *mut *mut glib::GError,
        ) -> gboolean,
    >,
    pub _polkit_reserved0: Option<unsafe extern "C" fn()>,
    pub _polkit_reserved1: Option<unsafe extern "C" fn()>,
    pub _polkit_reserved2: Option<unsafe extern "C" fn()>,
    pub _polkit_reserved3: Option<unsafe extern "C" fn()>,
    pub _polkit_reserved4: Option<unsafe extern "C" fn()>,
    pub _polkit_reserved5: Option<unsafe extern "C" fn()>,
    pub _polkit_reserved6: Option<unsafe extern "C" fn()>,
    pub _polkit_reserved7: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for PolkitAgentListenerClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PolkitAgentListenerClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .field("initiate_authentication", &self.initiate_authentication)
            .field(
                "initiate_authentication_finish",
                &self.initiate_authentication_finish,
            )
            .field("_polkit_reserved0", &self._polkit_reserved0)
            .field("_polkit_reserved1", &self._polkit_reserved1)
            .field("_polkit_reserved2", &self._polkit_reserved2)
            .field("_polkit_reserved3", &self._polkit_reserved3)
            .field("_polkit_reserved4", &self._polkit_reserved4)
            .field("_polkit_reserved5", &self._polkit_reserved5)
            .field("_polkit_reserved6", &self._polkit_reserved6)
            .field("_polkit_reserved7", &self._polkit_reserved7)
            .finish()
    }
}

#[repr(C)]
pub struct _PolkitAgentSessionClass(c_void);

pub type PolkitAgentSessionClass = *mut _PolkitAgentSessionClass;

// Classes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PolkitAgentListener {
    pub parent_instance: gobject::GObject,
}

impl ::std::fmt::Debug for PolkitAgentListener {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PolkitAgentListener @ {:p}", self))
            .field("parent_instance", &self.parent_instance)
            .finish()
    }
}

#[repr(C)]
pub struct PolkitAgentSession(c_void);

impl ::std::fmt::Debug for PolkitAgentSession {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PolkitAgentSession @ {:p}", self))
            .finish()
    }
}

#[repr(C)]
pub struct PolkitAgentTextListener(c_void);

impl ::std::fmt::Debug for PolkitAgentTextListener {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PolkitAgentTextListener @ {:p}", self))
            .finish()
    }
}

#[link(name = "polkit-agent-1")]
extern "C" {

    //=========================================================================
    // PolkitAgentRegisterFlags
    //=========================================================================
    pub fn polkit_agent_register_flags_get_type() -> GType;

    //=========================================================================
    // PolkitAgentListener
    //=========================================================================
    pub fn polkit_agent_listener_get_type() -> GType;
    pub fn polkit_agent_listener_unregister(registration_handle: gpointer);
    pub fn polkit_agent_listener_initiate_authentication(
        listener: *mut PolkitAgentListener,
        action_id: *const c_char,
        message: *const c_char,
        icon_name: *const c_char,
        details: *mut polkit::PolkitDetails,
        cookie: *const c_char,
        identities: *mut glib::GList,
        cancellable: *mut gio::GCancellable,
        callback: gio::GAsyncReadyCallback,
        user_data: gpointer,
    );
    pub fn polkit_agent_listener_initiate_authentication_finish(
        listener: *mut PolkitAgentListener,
        res: *mut gio::GAsyncResult,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn polkit_agent_listener_register(
        listener: *mut PolkitAgentListener,
        flags: PolkitAgentRegisterFlags,
        subject: *mut polkit::PolkitSubject,
        object_path: *const c_char,
        cancellable: *mut gio::GCancellable,
        error: *mut *mut glib::GError,
    ) -> gpointer;
    pub fn polkit_agent_listener_register_with_options(
        listener: *mut PolkitAgentListener,
        flags: PolkitAgentRegisterFlags,
        subject: *mut polkit::PolkitSubject,
        object_path: *const c_char,
        options: *mut glib::GVariant,
        cancellable: *mut gio::GCancellable,
        error: *mut *mut glib::GError,
    ) -> gpointer;

    //=========================================================================
    // PolkitAgentSession
    //=========================================================================
    pub fn polkit_agent_session_get_type() -> GType;
    pub fn polkit_agent_session_new(
        identity: *mut polkit::PolkitIdentity,
        cookie: *const c_char,
    ) -> *mut PolkitAgentSession;
    pub fn polkit_agent_session_cancel(session: *mut PolkitAgentSession);
    pub fn polkit_agent_session_initiate(session: *mut PolkitAgentSession);
    pub fn polkit_agent_session_response(session: *mut PolkitAgentSession, response: *const c_char);

    //=========================================================================
    // PolkitAgentTextListener
    //=========================================================================
    pub fn polkit_agent_text_listener_get_type() -> GType;
    pub fn polkit_agent_text_listener_new(
        cancellable: *mut gio::GCancellable,
        error: *mut *mut glib::GError,
    ) -> *mut PolkitAgentListener;

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn polkit_agent_register_listener(
        listener: *mut PolkitAgentListener,
        subject: *mut polkit::PolkitSubject,
        object_path: *const c_char,
        error: *mut *mut glib::GError,
    ) -> gboolean;

}