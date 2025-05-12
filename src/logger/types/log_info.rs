use crate::logger::types::log_color::LogColor;
use crate::logger::types::log_severity::LogSeverity;

/// [`LogInfo`] is a collection of information for logging messages of varying [`LogSeverity`].
pub struct LogInfo {
    /// The severity of the log, which denotes [`LogInfo::color`].
    pub severity: LogSeverity,

    /// The color of the log, based on [`LogInfo::severity`].
    pub color: LogColor,
}
