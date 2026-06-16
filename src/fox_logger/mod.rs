use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

// ============ 日志级别 ============
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
    Fatal = 5,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Fatal => "FATAL",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "TRACE" => Some(LogLevel::Trace),
            "DEBUG" => Some(LogLevel::Debug),
            "INFO" => Some(LogLevel::Info),
            "WARN" => Some(LogLevel::Warn),
            "ERROR" => Some(LogLevel::Error),
            "FATAL" => Some(LogLevel::Fatal),
            _ => None,
        }
    }
}

// ============ 日志配置 ============
#[derive(Clone)]
pub struct LoggerConfig {
    pub min_level: LogLevel,
    pub enable_console: bool,
    pub enable_file: bool,
    pub log_file_path: Option<PathBuf>,
    pub enable_timestamp: bool,
    pub enable_file_location: bool,
    pub enable_colors: bool,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            min_level: LogLevel::Info,
            enable_console: true,
            enable_file: false,
            log_file_path: None,
            enable_timestamp: true,
            enable_file_location: true,
            enable_colors: true,
        }
    }
}

// ============ 日志记录器 ============
pub struct Logger {
    config: LoggerConfig,
    file_handle: Option<Mutex<File>>,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            config: LoggerConfig::default(),
            file_handle: None,
        }
    }

    pub fn with_config(mut self, config: LoggerConfig) -> Self {
        if config.enable_file {
            if let Some(ref path) = config.log_file_path {
                match OpenOptions::new().create(true).append(true).open(path) {
                    Ok(file) => {
                        self.file_handle = Some(Mutex::new(file));
                    }
                    Err(..) => {}
                }
            }
        }
        self.config = config;
        self
    }

    pub fn set_min_level(&mut self, level: LogLevel) {
        self.config.min_level = level;
    }

    pub fn enable_console(&mut self, enable: bool) {
        self.config.enable_console = enable;
    }

    pub fn enable_file(&mut self, enable: bool) {
        if enable && self.file_handle.is_none() {
            if let Some(ref path) = self.config.log_file_path {
                if let Ok(file) = OpenOptions::new().create(true).append(true).open(path) {
                    self.file_handle = Some(Mutex::new(file));
                }
            }
        }
        self.config.enable_file = enable;
    }

    pub fn set_log_file(&mut self, path: PathBuf) -> Result<(), String> {
        self.config.log_file_path = Some(path.clone());
        match OpenOptions::new().create(true).append(true).open(&path) {
            Ok(file) => {
                self.file_handle = Some(Mutex::new(file));
                Ok(())
            }
            Err(e) => Err(format!("Failed to open log file: {}", e)),
        }
    }

    // ===== 核心日志方法 =====
    pub fn log(
        &self,
        level: LogLevel,
        file: &str,
        line: u32,
        column: u32,
        args: std::fmt::Arguments,
    ) {
        if level < self.config.min_level {
            return;
        }

        let timestamp = if self.config.enable_timestamp {
            format!("[{}] ", Self::current_timestamp())
        } else {
            String::new()
        };

        let level_str = level.as_str();
        let colored_level = if self.config.enable_colors {
            Self::colorize_level(level_str, level)
        } else {
            level_str.to_string()
        };

        let location = if self.config.enable_file_location {
            format!(" {}:{}:{}", file, line, column)
        } else {
            String::new()
        };

        let message = format!("{}{}{} {}", timestamp, colored_level, location, args);

        // 输出到控制台
        if self.config.enable_console {
            println!("{}", message);
        }

        // 输出到文件
        if self.config.enable_file {
            if let Some(ref handle) = self.file_handle {
                let _ = handle.lock().unwrap().write_all(message.as_bytes());
                let _ = handle.lock().unwrap().write_all(b"\n");
            }
        }
    }

    // ===== 便捷方法 =====
    pub fn trace(&self, file: &str, line: u32, column: u32, args: std::fmt::Arguments) {
        self.log(LogLevel::Trace, file, line, column, args);
    }

    pub fn debug(&self, file: &str, line: u32, column: u32, args: std::fmt::Arguments) {
        self.log(LogLevel::Debug, file, line, column, args);
    }

    pub fn info(&self, file: &str, line: u32, column: u32, args: std::fmt::Arguments) {
        self.log(LogLevel::Info, file, line, column, args);
    }

    pub fn warn(&self, file: &str, line: u32, column: u32, args: std::fmt::Arguments) {
        self.log(LogLevel::Warn, file, line, column, args);
    }

    pub fn error(&self, file: &str, line: u32, column: u32, args: std::fmt::Arguments) {
        self.log(LogLevel::Error, file, line, column, args);
    }

    pub fn fatal(&self, file: &str, line: u32, column: u32, args: std::fmt::Arguments) {
        self.log(LogLevel::Fatal, file, line, column, args);
    }

    // ===== 辅助函数 =====
    fn current_timestamp() -> String {
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).unwrap_or_default();
        let secs = duration.as_secs();
        let millis = duration.subsec_millis();

        // 简单时间格式化（使用 UTC）
        let seconds = secs % 60;
        let minutes = (secs / 60) % 60;
        let hours = (secs / 3600) % 24;

        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
            1970,
            1,
            1 + (secs / 86400) as u32, // 注意：这是简化版，不考虑闰年
            hours,
            minutes,
            seconds,
            millis
        )
    }

    fn colorize_level(level_str: &str, level: LogLevel) -> String {
        match level {
            LogLevel::Trace => format!("\x1b[90m{}\x1b[0m", level_str), // 灰色
            LogLevel::Debug => format!("\x1b[36m{}\x1b[0m", level_str), // 青色
            LogLevel::Info => format!("\x1b[32m{}\x1b[0m", level_str),  // 绿色
            LogLevel::Warn => format!("\x1b[33m{}\x1b[0m", level_str),  // 黄色
            LogLevel::Error => format!("\x1b[31m{}\x1b[0m", level_str), // 红色
            LogLevel::Fatal => format!("\x1b[35m{}\x1b[0m", level_str), // 紫色
        }
    }
}

// ============ 全局单例 ============
use std::sync::OnceLock;

static GLOBAL_LOGGER: OnceLock<Logger> = OnceLock::new();

/// 初始化全局日志器
pub fn init_logger(logger: Logger) -> Result<(), String> {
    GLOBAL_LOGGER
        .set(logger)
        .map_err(|_| "Global logger already initialized".to_string())
}

/// 获取全局日志器
pub fn get_logger() -> &'static Logger {
    GLOBAL_LOGGER
        .get()
        .expect("Logger not initialized. Call init_logger() first.")
}

/// 初始化默认日志器（仅控制台输出）
pub fn init_default() {
    let logger = Logger::new();
    let _ = init_logger(logger);
}

/// 初始化带文件输出的日志器
pub fn init_with_file(path: &str) -> Result<(), String> {
    let mut logger = Logger::new();
    logger.set_log_file(PathBuf::from(path))?;
    logger.enable_file(true);
    init_logger(logger)
}

// ============ 日志宏 ============
/// 记录 TRACE 级别日志
#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        $crate::get_logger().trace(file!(), line!(), column!(), format_args!($($arg)*))
    };
}

/// 记录 DEBUG 级别日志
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::get_logger().debug(file!(), line!(), column!(), format_args!($($arg)*))
    };
}

/// 记录 INFO 级别日志
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::get_logger().info(file!(), line!(), column!(), format_args!($($arg)*))
    };
}

/// 记录 WARN 级别日志
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::get_logger().warn(file!(), line!(), column!(), format_args!($($arg)*))
    };
}

/// 记录 ERROR 级别日志
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::get_logger().error(file!(), line!(), column!(), format_args!($($arg)*))
    };
}

/// 记录 FATAL 级别日志
#[macro_export]
macro_rules! log_fatal {
    ($($arg:tt)*) => {
        $crate::get_logger().fatal(file!(), line!(), column!(), format_args!($($arg)*))
    };
}

/// 带错误对象的日志（便捷宏）
#[macro_export]
macro_rules! log_error_with {
    ($e:expr) => {
        $crate::log_error!("{}:{} 操作失败: {}", file!(), line!(), $e)
    };
    ($e:expr, $($arg:tt)*) => {
        $crate::log_error!("{}:{} 操作失败: {} - {}", file!(), line!(), $e, format!($($arg)*))
    };
}
