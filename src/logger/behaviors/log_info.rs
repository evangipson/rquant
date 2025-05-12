use crate::logger::types::{log_color::LogColor, log_info::LogInfo, log_severity::LogSeverity};

/// Implementing [`LogInfo`].
impl LogInfo {
    /// [`LogInfo::new`] will create a new [`LogInfo`] to format a log message using a
    /// [`LogSeverity`] and [`LogColor`].
    ///
    /// # Example
    /// [`LogInfo::new`] can be used to create a new [`LogInfo`]:
    /// ```rust
    /// use rquant::logger::types::{
    ///     log_color::LogColor,
    ///     log_info::LogInfo,
    ///     log_severity::LogSeverity
    /// };
    ///
    /// fn create_log_info() -> LogInfo {
    ///     LogInfo::new(LogSeverity::Info, LogColor::Green)
    /// }
    /// ```
    pub fn new(severity: LogSeverity, color: LogColor) -> Self {
        LogInfo { severity, color }
    }
}
