use crate::logger::types::log_severity::LogSeverity;

/// Implementing [`LogSeverity`].
impl LogSeverity {
    /// [`LogSeverity::get_name`] will get the name of a [`LogSeverity`].
    ///
    /// # Example
    /// [`LogSeverity::get_name`] can be used to get the name of a [`LogSeverity`]:
    /// ```rust
    /// use rquant::logger::types::log_severity::LogSeverity;
    ///
    /// fn get_severity_name() -> String {
    ///     LogSeverity::Info.get_name().to_string()
    /// }
    /// ```
    pub fn get_name(&self) -> &str {
        match self {
            LogSeverity::Debug => "Debug",
            LogSeverity::Info => "Info",
            LogSeverity::Warning => "Warning",
            LogSeverity::Error => "Error",
        }
    }
}
