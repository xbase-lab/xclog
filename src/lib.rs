// TODO: remove experiential code
//! xcodebuild command wrapper, runner and log parser.
// #![deny(future_incompatible)]
// #![deny(nonstandard_style)]
// #![deny(missing_docs)]
// #![deny(rustdoc::broken_intra_doc_links)]

/// Regex based Parser
pub mod parser;
#[cfg(feature = "manual")]
pub mod runner;
