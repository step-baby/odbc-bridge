#![deny(missing_debug_implementations)]

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate log;

pub extern crate dameng_helper;
pub extern crate pg_helper;

pub use odbc_api;

pub mod bridge;
pub mod error;
pub mod executor;
pub mod extension;

pub use odbc_common::Print;

pub trait Convert<T>: Sized {
    fn convert(self) -> T;
}

pub trait TryConvert<T>: Sized {
    type Error;
    fn try_convert(self) -> Result<T, Self::Error>;
}
