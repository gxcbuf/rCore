use core::fmt;

use log::{Level, LevelFilter, Log, Metadata, Record};

use crate::console;

struct Logger;

pub fn init() {
    static LOGGER: Logger = Logger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("info") => LevelFilter::Info,
        Some("error") => LevelFilter::Error,
        Some("warn") => LevelFilter::Warn,
        Some("debug") => LevelFilter::Debug,
        Some("trace") => LevelFilter::Trace,
        _ => LevelFilter::Off,
    });
}

#[macro_export]
macro_rules! with_color {
    ($args: ident, $color: ident) => {
        format_args!("\x1b[{}m{}\x1b[0m\n", $color as u8, $args)
    };
}

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let color = level_to_color(record.level());
            print_with_color(
                format_args!("[{}] {}", record.level(), record.args()),
                color,
            );
        }
    }

    fn flush(&self) {}
}

fn print_with_color(args: fmt::Arguments, color: u8) {
    console::print(with_color!(args, color));
}

fn level_to_color(level: Level) -> u8 {
    match level {
        Level::Error => 31,
        Level::Warn => 93,
        Level::Info => 34,
        Level::Debug => 32,
        Level::Trace => 90,
    }
}
