//! Backtrace Integration and utilities for sentry.
//!
//! Exposes functions to capture, process and convert/parse stacktraces, as well
//! as integrations to process event stacktraces.

#![doc(html_favicon_url = "https://sentry-brand.storage.googleapis.com/favicon.ico")]
#![doc(html_logo_url = "https://sentry-brand.storage.googleapis.com/sentry-glyph-black.png")]
#![warn(missing_docs)]

mod integration;
mod parse;
mod process;
mod trim;
mod utils;

pub use crate::integration::{
    current_thread, AttachStacktraceIntegration, ProcessStacktraceIntegration,
};
pub use crate::parse::parse_stacktrace;
pub use crate::process::{backtrace_to_stacktrace, process_event_stacktrace};
pub use crate::trim::trim_stacktrace;
pub use sentry_core::protocol::{Frame, Stacktrace};

/// Strategy to use when creating the backtrace
#[derive(Default, Debug, Copy, Clone)]
pub enum BacktraceResolutionStrategy {
    /// Backtrace with resolved symbols
    #[default]
    SymbolsResolved,
    /// Backtrace with unresolved addresses only
    Unresolved,
}

/// Returns the current backtrace as sentry stacktrace.
pub fn current_stacktrace(strategy: BacktraceResolutionStrategy) -> Option<Stacktrace> {
    let backtrace = match strategy {
        BacktraceResolutionStrategy::SymbolsResolved => backtrace::Backtrace::new(),
        BacktraceResolutionStrategy::Unresolved => backtrace::Backtrace::new_unresolved(),
    };
    backtrace_to_stacktrace(&backtrace)
}
