#![allow(deprecated)]
#![allow(dead_code)]

extern crate gio_sys;
extern crate glib_sys;
extern crate gobject_sys;
extern crate polkit_sys;
#[macro_use]
extern crate glib;
extern crate gio;
#[macro_use]
extern crate bitflags;

mod auto;
pub use auto::*;
