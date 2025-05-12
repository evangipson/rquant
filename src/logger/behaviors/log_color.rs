use crate::logger::types::log_color::LogColor;

/// Implementing [`LogColor`].
impl LogColor {
    /// [`LogColor::RESET`] represents an escape code for the color white.
    pub const RESET: &str = LogColor::White.get_escape_code();

    /// [`LogColor::RESET`] represents an escape code for the color grey.
    pub const GREY: &str = LogColor::Grey.get_escape_code();

    /// [`LogColor::get_escape_code`] will get a hexidecimal ANSI escape code for
    /// any [`LogColor`].
    ///
    /// # Example
    /// [`LogColor::get_escape_code`] can be used to get an escape code for a [`LogColor`]:
    /// ```rust
    /// use rquant::logger::types::log_color::LogColor;
    ///
    /// fn get_red_escape_code() -> String {
    ///     LogColor::Red.get_escape_code().to_string()
    /// }
    /// ```
    pub const fn get_escape_code(&self) -> &str {
        match self {
            LogColor::Red => "\x1b[91m",
            LogColor::Yellow => "\x1b[93m",
            LogColor::Green => "\x1b[92m",
            LogColor::Cyan => "\x1b[96m",
            LogColor::Grey => "\x1b[90m",
            LogColor::White => "\x1b[97m",
        }
    }
}
