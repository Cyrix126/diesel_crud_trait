#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
/// module of errors used by the CRUD methods.
pub mod error;

#[cfg(feature = "derive")]
/// The derive macro to apply to model struct.
pub extern crate diesel_crud_trait_derive;
