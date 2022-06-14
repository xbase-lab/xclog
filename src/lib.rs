#![deny(missing_docs)]
#![deny(future_incompatible)]
#![deny(nonstandard_style)]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../README.md")]

mod build_settings;
mod compile;
mod logger;
pub mod parser;
pub use build_settings::*;
pub use compile::*;
pub use logger::*;
