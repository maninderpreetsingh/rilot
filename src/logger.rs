use log::{Level, LevelFilter, Metadata, Record};
use std::io::{self, Write};
use chrono::Local;

pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let now = Local::now();
            let timestamp = now.format("%Y-%m-%d %H:%M:%S%.3f");

            let level_str = match record.level() {
                Level::Error => "❌ ERROR",
                Level::Warn => "⚠️ WARN",
                Level::Info => "ℹ️ INFO",
                Level::Debug => "🔍 DEBUG",
                Level::Trace => "🔎 TRACE",
            };

            let target = if record.target().starts_with("rilot") {
                record.target()
            } else {
                "rilot"
            };

            let args = record.args();
            let file = record.file().unwrap_or("unknown");
            let line = record.line().unwrap_or(0);

            let output = format!(
                "{} {} [{}:{}] {}: {}\n",
                timestamp,
                level_str,
                file,
                line,
                target,
                args
            );

            let _ = io::stderr().write_all(output.as_bytes());
        }
    }

    fn flush(&self) {
        let _ = io::stderr().flush();
    }
}

pub fn init() {
    let env = env_logger::Env::default()
        .filter_or("RILOT_LOG", "info")
        .write_style_or("RILOT_LOG_STYLE", "always");

    let level = env_logger::Builder::from_env(env)
        .format_timestamp(None)
        .format_module_path(false)
        .format_target(false)
        .build()
        .filter();

    log::set_boxed_logger(Box::new(Logger))
        .map(|()| log::set_max_level(level))
        .expect("Failed to initialize logger");
}