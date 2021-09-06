// Generated by gir (https://github.com/gtk-rs/gir @ f64f90a)
// from /usr/local/share/gir-1.0 (@ ???)
// DO NOT EDIT

mod action_description;
pub use self::action_description::ActionDescription;

mod authority;
pub use self::authority::Authority;

mod authorization_result;
pub use self::authorization_result::AuthorizationResult;

mod details;
pub use self::details::Details;

mod identity;
pub use self::identity::{Identity, NONE_IDENTITY};

mod permission;
pub use self::permission::Permission;

mod subject;
pub use self::subject::{Subject, NONE_SUBJECT};

mod system_bus_name;
pub use self::system_bus_name::SystemBusName;

mod temporary_authorization;
pub use self::temporary_authorization::TemporaryAuthorization;

mod unix_group;
pub use self::unix_group::UnixGroup;

mod unix_netgroup;
pub use self::unix_netgroup::UnixNetgroup;

mod unix_process;
pub use self::unix_process::UnixProcess;

mod unix_session;
pub use self::unix_session::UnixSession;

mod unix_user;
pub use self::unix_user::UnixUser;

mod enums;
pub use self::enums::Error;
pub use self::enums::ImplicitAuthorization;

mod flags;
pub use self::flags::AuthorityFeatures;
pub use self::flags::CheckAuthorizationFlags;

#[doc(hidden)]
pub mod traits {
    pub use super::identity::IdentityExt;
    pub use super::subject::SubjectExt;
}
