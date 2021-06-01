// Generated by gir (https://github.com/gtk-rs/gir @ 796942c)
// from /usr/local/share/gir-1.0 (@ ???)
// DO NOT EDIT

use glib::error::ErrorDomain;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::Quark;
use glib::StaticType;
use glib::Type;
use std::ffi::CStr;
use std::fmt;
use std::mem;

/// Possible error when using PolicyKit.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "PolkitError")]
pub enum Error {
    /// The operation failed.
    #[doc(alias = "POLKIT_ERROR_FAILED")]
    Failed,
    /// The operation was cancelled.
    #[doc(alias = "POLKIT_ERROR_CANCELLED")]
    Cancelled,
    /// Operation is not supported.
    #[doc(alias = "POLKIT_ERROR_NOT_SUPPORTED")]
    NotSupported,
    /// Not authorized to perform operation.
    #[doc(alias = "POLKIT_ERROR_NOT_AUTHORIZED")]
    NotAuthorized,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error::{}",
            match *self {
                Self::Failed => "Failed",
                Self::Cancelled => "Cancelled",
                Self::NotSupported => "NotSupported",
                Self::NotAuthorized => "NotAuthorized",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for Error {
    type GlibType = ffi::PolkitError;

    fn into_glib(self) -> ffi::PolkitError {
        match self {
            Self::Failed => ffi::POLKIT_ERROR_FAILED,
            Self::Cancelled => ffi::POLKIT_ERROR_CANCELLED,
            Self::NotSupported => ffi::POLKIT_ERROR_NOT_SUPPORTED,
            Self::NotAuthorized => ffi::POLKIT_ERROR_NOT_AUTHORIZED,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::PolkitError> for Error {
    unsafe fn from_glib(value: ffi::PolkitError) -> Self {
        match value {
            ffi::POLKIT_ERROR_FAILED => Self::Failed,
            ffi::POLKIT_ERROR_CANCELLED => Self::Cancelled,
            ffi::POLKIT_ERROR_NOT_SUPPORTED => Self::NotSupported,
            ffi::POLKIT_ERROR_NOT_AUTHORIZED => Self::NotAuthorized,
            value => Self::__Unknown(value),
        }
    }
}

impl ErrorDomain for Error {
    fn domain() -> Quark {
        unsafe { from_glib(ffi::polkit_error_quark()) }
    }

    fn code(self) -> i32 {
        self.into_glib()
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            ffi::POLKIT_ERROR_FAILED => Some(Self::Failed),
            ffi::POLKIT_ERROR_CANCELLED => Some(Self::Cancelled),
            ffi::POLKIT_ERROR_NOT_SUPPORTED => Some(Self::NotSupported),
            ffi::POLKIT_ERROR_NOT_AUTHORIZED => Some(Self::NotAuthorized),
            _ => Some(Self::Failed),
        }
    }
}

impl StaticType for Error {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::polkit_error_get_type()) }
    }
}

impl glib::value::ValueType for Error {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for Error {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for Error {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

/// Possible implicit authorizations.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "PolkitImplicitAuthorization")]
pub enum ImplicitAuthorization {
    /// Unknown whether the subject is authorized, never returned in any public API.
    #[doc(alias = "POLKIT_IMPLICIT_AUTHORIZATION_UNKNOWN")]
    Unknown,
    /// Subject is not authorized.
    #[doc(alias = "POLKIT_IMPLICIT_AUTHORIZATION_NOT_AUTHORIZED")]
    NotAuthorized,
    /// Authentication is required.
    #[doc(alias = "POLKIT_IMPLICIT_AUTHORIZATION_AUTHENTICATION_REQUIRED")]
    AuthenticationRequired,
    /// Authentication as an administrator is required.
    #[doc(alias = "POLKIT_IMPLICIT_AUTHORIZATION_ADMINISTRATOR_AUTHENTICATION_REQUIRED")]
    AdministratorAuthenticationRequired,
    /// Authentication is required. If the authorization is obtained, it is retained.
    #[doc(alias = "POLKIT_IMPLICIT_AUTHORIZATION_AUTHENTICATION_REQUIRED_RETAINED")]
    AuthenticationRequiredRetained,
    /// Authentication as an administrator is required. If the authorization is obtained, it is retained.
    #[doc(alias = "POLKIT_IMPLICIT_AUTHORIZATION_ADMINISTRATOR_AUTHENTICATION_REQUIRED_RETAINED")]
    AdministratorAuthenticationRequiredRetained,
    /// The subject is authorized
    #[doc(alias = "POLKIT_IMPLICIT_AUTHORIZATION_AUTHORIZED")]
    Authorized,
    #[doc(hidden)]
    __Unknown(i32),
}

impl ImplicitAuthorization {
    #[doc(alias = "polkit_implicit_authorization_from_string")]
    pub fn from_string(string: &str) -> Option<ImplicitAuthorization> {
        unsafe {
            let mut out_implicit_authorization = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::polkit_implicit_authorization_from_string(
                string.to_glib_none().0,
                out_implicit_authorization.as_mut_ptr(),
            ));
            let out_implicit_authorization = out_implicit_authorization.assume_init();
            if ret {
                Some(from_glib(out_implicit_authorization))
            } else {
                None
            }
        }
    }

    pub fn to_str<'a>(self) -> &'a str {
        unsafe {
            CStr::from_ptr(
                ffi::polkit_implicit_authorization_to_string(self.into_glib())
                    .as_ref()
                    .expect("polkit_implicit_authorization_to_string returned NULL"),
            )
            .to_str()
            .expect("polkit_implicit_authorization_to_string returned an invalid string")
        }
    }
}

impl fmt::Display for ImplicitAuthorization {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_str())
    }
}

#[doc(hidden)]
impl IntoGlib for ImplicitAuthorization {
    type GlibType = ffi::PolkitImplicitAuthorization;

    fn into_glib(self) -> ffi::PolkitImplicitAuthorization {
        match self {
            Self::Unknown => ffi::POLKIT_IMPLICIT_AUTHORIZATION_UNKNOWN,
            Self::NotAuthorized => ffi::POLKIT_IMPLICIT_AUTHORIZATION_NOT_AUTHORIZED,
            Self::AuthenticationRequired => {
                ffi::POLKIT_IMPLICIT_AUTHORIZATION_AUTHENTICATION_REQUIRED
            }
            Self::AdministratorAuthenticationRequired => {
                ffi::POLKIT_IMPLICIT_AUTHORIZATION_ADMINISTRATOR_AUTHENTICATION_REQUIRED
            }
            Self::AuthenticationRequiredRetained => {
                ffi::POLKIT_IMPLICIT_AUTHORIZATION_AUTHENTICATION_REQUIRED_RETAINED
            }
            Self::AdministratorAuthenticationRequiredRetained => {
                ffi::POLKIT_IMPLICIT_AUTHORIZATION_ADMINISTRATOR_AUTHENTICATION_REQUIRED_RETAINED
            }
            Self::Authorized => ffi::POLKIT_IMPLICIT_AUTHORIZATION_AUTHORIZED,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::PolkitImplicitAuthorization> for ImplicitAuthorization {
    unsafe fn from_glib(value: ffi::PolkitImplicitAuthorization) -> Self {
        match value {
            ffi::POLKIT_IMPLICIT_AUTHORIZATION_UNKNOWN => Self::Unknown,
            ffi::POLKIT_IMPLICIT_AUTHORIZATION_NOT_AUTHORIZED => Self::NotAuthorized,
            ffi::POLKIT_IMPLICIT_AUTHORIZATION_AUTHENTICATION_REQUIRED => {
                Self::AuthenticationRequired
            }
            ffi::POLKIT_IMPLICIT_AUTHORIZATION_ADMINISTRATOR_AUTHENTICATION_REQUIRED => {
                Self::AdministratorAuthenticationRequired
            }
            ffi::POLKIT_IMPLICIT_AUTHORIZATION_AUTHENTICATION_REQUIRED_RETAINED => {
                Self::AuthenticationRequiredRetained
            }
            ffi::POLKIT_IMPLICIT_AUTHORIZATION_ADMINISTRATOR_AUTHENTICATION_REQUIRED_RETAINED => {
                Self::AdministratorAuthenticationRequiredRetained
            }
            ffi::POLKIT_IMPLICIT_AUTHORIZATION_AUTHORIZED => Self::Authorized,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for ImplicitAuthorization {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::polkit_implicit_authorization_get_type()) }
    }
}

impl glib::value::ValueType for ImplicitAuthorization {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for ImplicitAuthorization {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for ImplicitAuthorization {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}
