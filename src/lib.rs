#![feature(
    generic_const_exprs,
    maybe_uninit_array_assume_init,
    portable_simd,
    new_zeroed_alloc
)]
#![allow(incomplete_features)]
#![warn(clippy::all)]
#![allow(clippy::result_unit_err)]

extern crate alloc;
pub extern crate cfg_if;
extern crate self as emu_utils;

pub use emu_utils_macros::*;

#[cfg(all(feature = "app", target_os = "macos", app_bundle))]
#[macro_use]
extern crate objc;

mod bounded;
mod fifo;
pub use fifo::Fifo;
mod mem;
pub use mem::*;
mod savestate;
pub use savestate::*;
#[cfg(feature = "app")]
pub mod app;
pub mod schedule;
#[cfg(feature = "triple-buffer")]
pub mod triple_buffer;

pub mod mem_prelude {
    pub use crate::{ByteSlice, ByteMutSlice, ByteMutSliceOwnedPtr};
    pub use crate::{BoxedByteSlice, Bytes, OwnedByteSliceCellPtr, OwnedBytesCellPtr};
    pub use crate::MemValue;
}
