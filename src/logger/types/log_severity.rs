/// [`LogSeverity`] denotes levels of logging severity.
pub enum LogSeverity {
    /// [`LogSeverity::Debug`] represents the "debug" logging severity level, and is
    /// the least severe logging level.
    Debug,

    /// [`LogSeverity::Info`] represents the "informational" logging severity level.
    Info,

    /// [`LogSeverity::Warning`] represents the "warning" logging severity level.
    Warning,

    /// [`LogSeverity::Error`] represents the "error" logging severity level, and is
    /// the most severe logging level.
    Error,
}
