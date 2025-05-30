/// [`log_debug!`](crate::log_debug) is a wrapper function to [`Logger::debug`](crate::logger::types::logger::Logger::debug)
/// which will provide the [`Debug`](crate::logger::types::log_severity::LogSeverity::Debug) log severity level and some additional
/// helpful information like file and line number.
///
/// # Example
/// [`log_debug!`](crate::log_debug) can be used to pring out a [`Debug`](crate::logger::types::log_severity::LogSeverity::Debug) message
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
        $crate::logger::types::logger::Logger::debug(&format!($($arg)*), file!(), line!());
    };
}

/// [`log_info!`](crate::log_info) is a wrapper function to [`Logger::info`](crate::logger::types::logger::Logger::info)
/// which will provide the [`Info`](crate::logger::types::log_severity::LogSeverity::Info) log severity level and some additional
/// helpful information like file and line number.
///
/// # Example
/// [`log_info!`](crate::log_info) can be used to pring out an [`Info`](crate::logger::types::log_severity::LogSeverity::Info)
/// message to the console:
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
        $crate::logger::types::logger::Logger::info(&format!($($arg)*), file!(), line!());
    };
}

/// [`log_warn!`](crate::log_warn) is a wrapper function to [`Logger::warn`](crate::logger::types::logger::Logger::warn)
/// which will provide the [`Warning`](crate::logger::types::log_severity::LogSeverity::Warning) log severity level and some additional
/// helpful information like file and line number.
///
/// # Example
/// [`log_warn!`](crate::log_warn) can be used to pring out a [`Warning`](crate::logger::types::log_severity::LogSeverity::Warning)
/// message to the console:
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
        $crate::logger::types::logger::Logger::warn(&format!($($arg)*), file!(), line!());
    };
}

/// [`log_error!`](crate::log_error) is a wrapper function to [`Logger::error`](crate::logger::types::logger::Logger::error)
/// which will provide the [`Error`](crate::logger::types::log_severity::LogSeverity::Error) log severity level and some additional
/// helpful information like file and line number.
///
/// # Example
/// [`log_error!`](crate::log_error) can be used to pring out an [`Error`](crate::logger::types::log_severity::LogSeverity::Error)
/// message to the console:
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
        $crate::logger::types::logger::Logger::error(&format!($($arg)*), file!(), line!());
    };
}
