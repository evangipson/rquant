/// [`log_debug!`] is a wrapper function to [`Logger::debug`](crate::types::logger::Logger::debug) which will
/// provide the [`Debug`](crate::types::logger::LogSeverity::Debug) log severity level and some additional
/// helpful information like file and line number.
///
/// # Example
/// [`log_debug!`] can be used to pring out a [`Debug`](crate::types::logger::LogSeverity::Debug) message
/// to the console:
/// ```rust
/// use rquant::log_debug;
///
/// fn log_message(message: &str) {
///     log_debug!("The debug message is: {message}");
/// }
/// ```
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::types::logger::Logger::debug(&format!($($arg)*), file!(), line!());
    };
}

/// [`log_info!`] is a wrapper function to [`Logger::info`](crate::types::logger::Logger::info) which will
/// provide the [`Info`](crate::types::logger::LogSeverity::Info) log severity level and some additional
/// helpful information like file and line number.
///
/// # Example
/// [`log_info!`] can be used to pring out an [`Info`](crate::types::logger::LogSeverity::Info) message
/// to the console:
/// ```rust
/// use rquant::log_info;
///
/// fn log_message(message: &str) {
///     log_info!("The informational message is: {message}");
/// }
/// ```
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::types::logger::Logger::info(&format!($($arg)*), file!(), line!());
    };
}

/// [`log_warn!`] is a wrapper function to [`Logger::warn`](crate::types::logger::Logger::warn) which will
/// provide the [`Warning`](crate::types::logger::LogSeverity::Warning) log severity level and some additional
/// helpful information like file and line number.
///
/// # Example
/// [`log_warn!`] can be used to pring out a [`Warning`](crate::types::logger::LogSeverity::Warning) message
/// to the console:
/// ```rust
/// use rquant::log_warn;
///
/// fn log_message(message: &str) {
///     log_warn!("The warning message is: {message}");
/// }
/// ```
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::types::logger::Logger::warn(&format!($($arg)*), file!(), line!());
    };
}

/// [`log_error!`] is a wrapper function to [`Logger::error`](crate::types::logger::Logger::error) which will
/// provide the [`Error`](crate::types::logger::LogSeverity::Error) log severity level and some additional
/// helpful information like file and line number.
///
/// # Example
/// [`log_error!`] can be used to pring out an [`Error`](crate::types::logger::LogSeverity::Error) message
/// to the console:
/// ```rust
/// use rquant::log_error;
///
/// fn log_message(message: &str) {
///     log_error!("The error message is: {message}");
/// }
/// ```
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::types::logger::Logger::error(&format!($($arg)*), file!(), line!());
    };
}
