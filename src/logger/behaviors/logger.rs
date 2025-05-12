use crate::logger::types::{
    log_color::LogColor, log_info::LogInfo, log_severity::LogSeverity, logger::Logger,
};

/// Implementing [`Logger`].
impl Logger {
    /// [`Logger::debug`] will log a debug message to the console using the [`LogSeverity::Debug`]
    /// severity level, and also provide some additional helpful information using `file` and `line_number`.
    ///
    /// # Example
    /// [`Logger::debug`] can be used to print out a [`LogSeverity::Debug`] message to the console:
    /// ```rust
    /// use rquant::logger::types::logger::Logger;
    ///
    /// fn log_debug_message(message: &str) {
    ///     Logger::debug(message, file!(), line!())
    /// }
    /// ```
    pub fn debug(message: &str, file: &str, line_number: u32) {
        Self::print_severity(LogInfo::new(LogSeverity::Debug, LogColor::Green));
        Self::print_file_info(file, line_number);
        Self::log(message);
    }

    /// [`Logger::info`] will log an informational message to the console using the [`LogSeverity::Info`]
    /// severity level, and also provide some additional helpful information using `file` and `line_number`.
    ///
    /// # Example
    /// [`Logger::info`] can be used to print out a [`LogSeverity::Info`] message to the console:
    /// ```rust
    /// use rquant::logger::types::logger::Logger;
    ///
    /// fn log_info_message(message: &str) {
    ///     Logger::info(message, file!(), line!())
    /// }
    /// ```
    pub fn info(message: &str, file: &str, line_number: u32) {
        Self::print_severity(LogInfo::new(LogSeverity::Info, LogColor::Cyan));
        Self::print_file_info(file, line_number);
        Self::log(message);
    }

    /// [`Logger::warn`] will log a warning message to the console using the [`LogSeverity::Warning`]
    /// severity level, and also provide some additional helpful information using `file` and `line_number`.
    ///
    /// # Example
    /// [`Logger::warn`] can be used to print out a [`LogSeverity::Warning`] message to the console:
    /// ```rust
    /// use rquant::logger::types::logger::Logger;
    ///
    /// fn log_warn_message(message: &str) {
    ///     Logger::warn(message, file!(), line!())
    /// }
    /// ```
    pub fn warn(message: &str, file: &str, line_number: u32) {
        Self::print_severity(LogInfo::new(LogSeverity::Warning, LogColor::Yellow));
        Self::print_file_info(file, line_number);
        Self::log(message);
    }

    /// [`Logger::error`] will log an error message to the console using the [`LogSeverity::Error`]
    /// severity level, and also provide some additional helpful information using `file` and `line_number`.
    ///
    /// # Example
    /// [`Logger::error`] can be used to print out a [`LogSeverity::Error`] message to the console:
    /// ```rust
    /// use rquant::logger::types::logger::Logger;
    ///
    /// fn log_error_message(message: &str) {
    ///     Logger::error(message, file!(), line!())
    /// }
    /// ```
    pub fn error(message: &str, file: &str, line_number: u32) {
        Self::print_severity(LogInfo::new(LogSeverity::Error, LogColor::Red));
        Self::print_file_info(file, line_number);
        Self::log(message);
    }

    /// [`Logger::print_severity`] will print out the severity context from the provided [`LogInfo`].
    fn print_severity(log_info: LogInfo) {
        let severity_name = log_info.severity.get_name();
        let severity_color = log_info.color.get_escape_code();
        print!("\n{severity_color}[{severity_name}]");
    }

    /// [`Logger::print_severity`] will print the `file` and `line_number`.
    fn print_file_info(file: &str, line_number: u32) {
        println!(" {}{file}:{line_number}{}", LogColor::GREY, LogColor::RESET);
    }

    /// [`Logger::log`] is the internal [`Logger`] function that prints out any message, regardless of
    /// [`LogSeverity`].
    fn log(message: &str) {
        if !message.is_empty() {
            println!("{message}");
        }
    }
}
