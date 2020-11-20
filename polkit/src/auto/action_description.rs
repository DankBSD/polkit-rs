// This file was generated by gir (https://github.com/gtk-rs/gir @ cea2f7c)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use glib::translate::*;
use glib::GString;
use polkit_sys;
use std::fmt;
use ImplicitAuthorization;

glib_wrapper! {
    pub struct ActionDescription(Object<polkit_sys::PolkitActionDescription, polkit_sys::PolkitActionDescriptionClass, ActionDescriptionClass>);

    match fn {
        get_type => || polkit_sys::polkit_action_description_get_type(),
    }
}

impl ActionDescription {
    /// Gets the action id for `self`.
    ///
    /// # Returns
    ///
    /// A string owned by `self`. Do not free.
    pub fn get_action_id(&self) -> GString {
        unsafe {
            from_glib_none(polkit_sys::polkit_action_description_get_action_id(self.to_glib_none().0))
        }
    }

    /// Get the value of the annotation with `key`.
    /// ## `key`
    /// An annotation key.
    ///
    /// # Returns
    ///
    /// `None` if there is no annoation with `key`,
    /// otherwise the annotation value owned by `self`. Do not
    /// free.
    pub fn get_annotation(&self, key: &str) -> Option<GString> {
        unsafe {
            from_glib_none(polkit_sys::polkit_action_description_get_annotation(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    /// Gets the keys of annotations defined in `self`.
    ///
    /// # Returns
    ///
    /// The annotation keys owned by `self`. Do not free.
    pub fn get_annotation_keys(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(polkit_sys::polkit_action_description_get_annotation_keys(self.to_glib_none().0))
        }
    }

    /// Gets the description used for `self`.
    ///
    /// # Returns
    ///
    /// A string owned by `self`. Do not free.
    pub fn get_description(&self) -> GString {
        unsafe {
            from_glib_none(polkit_sys::polkit_action_description_get_description(self.to_glib_none().0))
        }
    }

    /// Gets the icon name for `self`, if any.
    ///
    /// # Returns
    ///
    /// A string owned by `self`. Do not free.
    pub fn get_icon_name(&self) -> GString {
        unsafe {
            from_glib_none(polkit_sys::polkit_action_description_get_icon_name(self.to_glib_none().0))
        }
    }

    /// Gets the implicit authorization for `self` used for
    /// subjects in active sessions on a local console.
    ///
    /// # Returns
    ///
    /// A value from the `ImplicitAuthorization` enumeration.
    pub fn get_implicit_active(&self) -> ImplicitAuthorization {
        unsafe {
            from_glib(polkit_sys::polkit_action_description_get_implicit_active(self.to_glib_none().0))
        }
    }

    /// Gets the implicit authorization for `self` used for
    /// any subject.
    ///
    /// # Returns
    ///
    /// A value from the `ImplicitAuthorization` enumeration.
    pub fn get_implicit_any(&self) -> ImplicitAuthorization {
        unsafe {
            from_glib(polkit_sys::polkit_action_description_get_implicit_any(self.to_glib_none().0))
        }
    }

    /// Gets the implicit authorization for `self` used for
    /// subjects in inactive sessions on a local console.
    ///
    /// # Returns
    ///
    /// A value from the `ImplicitAuthorization` enumeration.
    pub fn get_implicit_inactive(&self) -> ImplicitAuthorization {
        unsafe {
            from_glib(polkit_sys::polkit_action_description_get_implicit_inactive(self.to_glib_none().0))
        }
    }

    /// Gets the message used for `self`.
    ///
    /// # Returns
    ///
    /// A string owned by `self`. Do not free.
    pub fn get_message(&self) -> GString {
        unsafe {
            from_glib_none(polkit_sys::polkit_action_description_get_message(self.to_glib_none().0))
        }
    }

    /// Gets the vendor name for `self`, if any.
    ///
    /// # Returns
    ///
    /// A string owned by `self`. Do not free.
    pub fn get_vendor_name(&self) -> GString {
        unsafe {
            from_glib_none(polkit_sys::polkit_action_description_get_vendor_name(self.to_glib_none().0))
        }
    }

    /// Gets the vendor URL for `self`, if any.
    ///
    /// # Returns
    ///
    /// A string owned by `self`. Do not free.
    pub fn get_vendor_url(&self) -> GString {
        unsafe {
            from_glib_none(polkit_sys::polkit_action_description_get_vendor_url(self.to_glib_none().0))
        }
    }
}

impl fmt::Display for ActionDescription {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ActionDescription")
    }
}