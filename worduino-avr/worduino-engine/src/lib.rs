#![no_std]

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]

extern crate avr_progmem;

pub mod prelude;
mod peripherals;
mod engine;
mod asset_format;
mod assets;

pub use prelude::*;
pub use peripherals::*;
pub use engine::*;
pub use assets::*;
