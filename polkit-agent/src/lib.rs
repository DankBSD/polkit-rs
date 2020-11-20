#![allow(deprecated)]
#![allow(dead_code)]

extern crate gio_sys;
extern crate glib_sys;
extern crate gobject_sys;
extern crate libc;
extern crate polkit_agent_sys;
extern crate polkit_sys;
#[macro_use]
extern crate glib;
extern crate gio;
extern crate polkit;
#[macro_use]
extern crate bitflags;

mod auto;
pub use auto::*;
