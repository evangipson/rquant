enum LogColor {
    Red,
    Yellow,
    Green,
    Cyan,
    Grey,
    White,
}

impl LogColor {
    const RESET: &str = LogColor::White.get_escape_code();
    const GREY: &str = LogColor::Grey.get_escape_code();

    const fn get_escape_code(&self) -> &str {
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

pub enum LogSeverity {
    Debug,
    Info,
    Warning,
    Error,
}

impl LogSeverity {
    fn get_name(&self) -> &str {
        match self {
            LogSeverity::Debug => "Debug",
            LogSeverity::Info => "Info",
            LogSeverity::Warning => "Warning",
            LogSeverity::Error => "Error",
        }
    }
}

struct LogInfo {
    severity: LogSeverity,
    color: LogColor,
}

impl LogInfo {
    fn new(severity: LogSeverity, color: LogColor) -> Self {
        LogInfo { severity, color }
    }
}

pub struct Logger;

/// [`Logger`] will display a log message at varying severity levels.
impl Logger {
    pub fn debug(message: &str, file: &str, line_number: u32) {
        Self::print_severity(LogInfo::new(LogSeverity::Debug, LogColor::Green));
        Self::print_file_info(file, line_number);
        Self::log(message);
    }

    pub fn info(message: &str, file: &str, line_number: u32) {
        Self::print_severity(LogInfo::new(LogSeverity::Info, LogColor::Cyan));
        Self::print_file_info(file, line_number);
        Self::log(message);
    }

    pub fn warn(message: &str, file: &str, line_number: u32) {
        Self::print_severity(LogInfo::new(LogSeverity::Warning, LogColor::Yellow));
        Self::print_file_info(file, line_number);
        Self::log(message);
    }

    pub fn error(message: &str, file: &str, line_number: u32) {
        Self::print_severity(LogInfo::new(LogSeverity::Error, LogColor::Red));
        Self::print_file_info(file, line_number);
        Self::log(message);
    }

    fn print_severity(log_info: LogInfo) {
        let severity_name = log_info.severity.get_name();
        let severity_color = log_info.color.get_escape_code();
        print!("\n{severity_color}[{severity_name}]");
    }

    fn print_file_info(file: &str, line_number: u32) {
        println!(" {}{file}:{line_number}{}", LogColor::GREY, LogColor::RESET);
    }

    fn log(message: &str) {
        if !message.is_empty() {
            println!("{message}");
        }
    }
}
