//! [`sod::Service`] logging implementations via [`log`](https://crates.io/crates/log).
//!
//! ## Service Impls
//! * [`LogDebugService`] logs [`Debug`] input at a configured log level to [`log::log`], returning the input as output.
//! * [`LogDisplayService`] logs [`Display`] input at a configured log level to [`log::log`], returning the input as output.
//!
//! ## Use Case
//! These [`Service`] impls are most useful for logging an event as it passes through a service chain.
//!
//! ## Example
//! ```
//! use sod::Service;
//! use sod_log::LogDisplayService;
//!
//! let logging_service = LogDisplayService::info("my event: ");
//! logging_service.process("hello world!").unwrap();
//! ```

use std::{
    borrow::Cow,
    fmt::{Debug, Display},
};

use log::Level;
use sod::Service;

/// A [`sod::Service`] that logs [`Debug`] input at a configured log level to [`log::log`], returning the input as output.
pub struct LogDebugService<'a> {
    level: Level,
    prefix: Cow<'a, str>,
}
impl<'a> LogDebugService<'a> {
    /// Log input at the given log level
    /// # Arguments
    /// * `level` - The log level
    /// * `prefix` - A prefix to prepend to the beginning of the log statment
    pub fn new<S: Into<Cow<'a, str>>>(level: Level, prefix: S) -> Self {
        Self {
            level,
            prefix: prefix.into(),
        }
    }
    /// Log as [`Level::Debug`]
    /// # Arguments
    /// * `prefix` - A prefix to prepend to the beginning of the log statment
    pub fn debug<S: Into<Cow<'a, str>>>(prefix: S) -> Self {
        Self::new(Level::Debug, prefix)
    }
    /// Log as [`Level::Error`]
    /// # Arguments
    /// * `prefix` - A prefix to prepend to the beginning of the log statment
    pub fn error<S: Into<Cow<'a, str>>>(prefix: S) -> Self {
        Self::new(Level::Error, prefix)
    }
    /// Log as [`Level::Info`]
    /// # Arguments
    /// * `prefix` - A prefix to prepend to the beginning of the log statment
    pub fn info<S: Into<Cow<'a, str>>>(prefix: S) -> Self {
        Self::new(Level::Info, prefix)
    }
    /// Log as [`Level::Trace`]
    /// # Arguments
    /// * `prefix` - A prefix to prepend to the beginning of the log statment
    pub fn trace<S: Into<Cow<'a, str>>>(prefix: S) -> Self {
        Self::new(Level::Trace, prefix)
    }
    /// Log as [`Level::Warn`]
    /// # Arguments
    /// * `prefix` - A prefix to prepend to the beginning of the log statment
    pub fn warn<S: Into<Cow<'a, str>>>(prefix: S) -> Self {
        Self::new(Level::Warn, prefix)
    }
}
impl<'a, T: Debug> Service<T> for LogDebugService<'a> {
    type Output = T;
    type Error = ();
    fn process(&self, input: T) -> Result<Self::Output, Self::Error> {
        log::log!(self.level, "{}{:?}", self.prefix, input);
        Ok(input)
    }
}

/// A [`sod::Service`] that logs optional [`Debug`] input when it is `Some(input)` at a configured log level to [`log::log`], returning the input as output.
///
/// This service is useful for logging an event as it passed through a service chain, while ignoring non-blocking service chains that may continuously process `None` in a tight loop.
pub struct LogOptionalDebugService<'a> {
    level: Level,
    prefix: Cow<'a, str>,
}
impl<'a> LogOptionalDebugService<'a> {
    /// Log input at the given log level
    /// # Arguments
    /// * `level` - The log level
    /// * `prefix` - A prefix to prepend to the beginning of the log statment
    pub fn new<S: Into<Cow<'a, str>>>(level: Level, prefix: S) -> Self {
        Self {
            level,
            prefix: prefix.into(),
        }
    }
    /// Log as [`Level::Debug`]
    /// # Arguments
    /// * `prefix` - A prefix to prepend to the beginning of the log statment
    pub fn debug<S: Into<Cow<'a, str>>>(prefix: S) -> Self {
        Self::new(Level::Debug, prefix)
    }
    /// Log as [`Level::Error`]
    /// # Arguments
    /// * `prefix` - A prefix to prepend to the beginning of the log statment
    pub fn error<S: Into<Cow<'a, str>>>(prefix: S) -> Self {
        Self::new(Level::Error, prefix)
    }
    /// Log as [`Level::Info`]
    /// # Arguments
    /// * `prefix` - A prefix to prepend to the beginning of the log statment
    pub fn info<S: Into<Cow<'a, str>>>(prefix: S) -> Self {
        Self::new(Level::Info, prefix)
    }
    /// Log as [`Level::Trace`]
    /// # Arguments
    /// * `prefix` - A prefix to prepend to the beginning of the log statment
    pub fn trace<S: Into<Cow<'a, str>>>(prefix: S) -> Self {
        Self::new(Level::Trace, prefix)
    }
    /// Log as [`Level::Warn`]
    /// # Arguments
    /// * `prefix` - A prefix to prepend to the beginning of the log statment
    pub fn warn<S: Into<Cow<'a, str>>>(prefix: S) -> Self {
        Self::new(Level::Warn, prefix)
    }
}
impl<'a, T: Debug> Service<Option<T>> for LogOptionalDebugService<'a> {
    type Output = Option<T>;
    type Error = ();
    fn process(&self, input: Option<T>) -> Result<Self::Output, Self::Error> {
        if let Some(input) = &input {
            log::log!(self.level, "{}{:?}", self.prefix, input);
        }
        Ok(input)
    }
}

/// A [`sod::Service`] that logs [`Display`] input at a configured log level to [`log::log`], returning the input as output.
///
/// This service is useful for logging an event as it passed through a service chain.
pub struct LogDisplayService<'a> {
    level: Level,
    prefix: Cow<'a, str>,
}
impl<'a> LogDisplayService<'a> {
    /// Log input at the given log level
    /// # Arguments
    /// * `level` - The log level
    /// * `prefix` - A prefix to prepend to the beginning of the log statment
    pub fn new<S: Into<Cow<'a, str>>>(level: Level, prefix: S) -> Self {
        Self {
            level,
            prefix: prefix.into(),
        }
    }
    /// Log as [`Level::Debug`]
    /// # Arguments
    /// * `prefix` - A prefix to prepend to the beginning of the log statment
    pub fn debug<S: Into<Cow<'a, str>>>(prefix: S) -> Self {
        Self::new(Level::Debug, prefix)
    }
    /// Log as [`Level::Error`]
    /// # Arguments
    /// * `prefix` - A prefix to prepend to the beginning of the log statment
    pub fn error<S: Into<Cow<'a, str>>>(prefix: S) -> Self {
        Self::new(Level::Error, prefix)
    }
    /// Log as [`Level::Info`]
    /// # Arguments
    /// * `prefix` - A prefix to prepend to the beginning of the log statment
    pub fn info<S: Into<Cow<'a, str>>>(prefix: S) -> Self {
        Self::new(Level::Info, prefix)
    }
    /// Log as [`Level::Trace`]
    /// # Arguments
    /// * `prefix` - A prefix to prepend to the beginning of the log statment
    pub fn trace<S: Into<Cow<'a, str>>>(prefix: S) -> Self {
        Self::new(Level::Trace, prefix)
    }
    /// Log as [`Level::Warn`]
    /// # Arguments
    /// * `prefix` - A prefix to prepend to the beginning of the log statment
    pub fn warn<S: Into<Cow<'a, str>>>(prefix: S) -> Self {
        Self::new(Level::Warn, prefix)
    }
}
impl<'a, T: Display> Service<T> for LogDisplayService<'a> {
    type Output = T;
    type Error = ();
    fn process(&self, input: T) -> Result<Self::Output, Self::Error> {
        log::log!(self.level, "{}{}", self.prefix, input);
        Ok(input)
    }
}

/// A [`sod::Service`] that logs optional [`Display`] input when it is `Some(input)` at a configured log level to [`log::log`], returning the input as output.
///
/// This service is useful for logging an event as it passed through a service chain, while ignoring non-blocking service chains that may continuously process `None` in a tight loop.
pub struct LogOptionalDisplayService<'a> {
    level: Level,
    prefix: Cow<'a, str>,
}
impl<'a> LogOptionalDisplayService<'a> {
    /// Log input at the given log level
    /// # Arguments
    /// * `level` - The log level
    /// * `prefix` - A prefix to prepend to the beginning of the log statment
    pub fn new<S: Into<Cow<'a, str>>>(level: Level, prefix: S) -> Self {
        Self {
            level,
            prefix: prefix.into(),
        }
    }
    /// Log as [`Level::Debug`]
    /// # Arguments
    /// * `prefix` - A prefix to prepend to the beginning of the log statment
    pub fn debug<S: Into<Cow<'a, str>>>(prefix: S) -> Self {
        Self::new(Level::Debug, prefix)
    }
    /// Log as [`Level::Error`]
    /// # Arguments
    /// * `prefix` - A prefix to prepend to the beginning of the log statment
    pub fn error<S: Into<Cow<'a, str>>>(prefix: S) -> Self {
        Self::new(Level::Error, prefix)
    }
    /// Log as [`Level::Info`]
    /// # Arguments
    /// * `prefix` - A prefix to prepend to the beginning of the log statment
    pub fn info<S: Into<Cow<'a, str>>>(prefix: S) -> Self {
        Self::new(Level::Info, prefix)
    }
    /// Log as [`Level::Trace`]
    /// # Arguments
    /// * `prefix` - A prefix to prepend to the beginning of the log statment
    pub fn trace<S: Into<Cow<'a, str>>>(prefix: S) -> Self {
        Self::new(Level::Trace, prefix)
    }
    /// Log as [`Level::Warn`]
    /// # Arguments
    /// * `prefix` - A prefix to prepend to the beginning of the log statment
    pub fn warn<S: Into<Cow<'a, str>>>(prefix: S) -> Self {
        Self::new(Level::Warn, prefix)
    }
}
impl<'a, T: Display> Service<Option<T>> for LogOptionalDisplayService<'a> {
    type Output = Option<T>;
    type Error = ();
    fn process(&self, input: Option<T>) -> Result<Self::Output, Self::Error> {
        if let Some(input) = &input {
            log::log!(self.level, "{}{}", self.prefix, input);
        }
        Ok(input)
    }
}
