#![feature(
    generic_const_exprs,
    maybe_uninit_uninit_array,
    maybe_uninit_array_assume_init,
    portable_simd,
    new_uninit
)]
#![allow(incomplete_features)]
#![warn(clippy::all)]
#![allow(clippy::result_unit_err)]

extern crate alloc;
pub extern crate cfg_if;
extern crate self as emu_utils;

pub use emu_utils_macros::*;

mod bounded;
mod fifo;
pub use fifo::Fifo;
mod mem;
pub use mem::*;
mod savestate;
pub use savestate::*;
pub mod schedule;
#[cfg(feature = "std")]
pub mod triple_buffer;
mod resource;
