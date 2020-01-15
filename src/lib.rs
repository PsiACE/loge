extern crate chrono;
pub extern crate env_logger;
extern crate log;

use chrono::Local;
use env_logger::Builder;

#[inline]
pub fn init() {
    try_init().unwrap();
}

pub fn try_init() -> Result<(), log::SetLoggerError> {
    try_init_custom_env("RUST_LOG")
}

pub fn init_custom_env(environment_variable_name: &str) {
    try_init_custom_env(environment_variable_name).unwrap();
}

pub fn try_init_custom_env(environment_variable_name: &str) -> Result<(), log::SetLoggerError> {
    let mut builder = formatted_builder();

    if let Ok(s) = ::std::env::var(environment_variable_name) {
        builder.parse_filters(&s);
    }

    builder.try_init()
}

pub fn formatted_builder() -> Builder {
    use std::io::Write;

    let mut builder = Builder::new();

    builder.format(|formatter, record| {
        writeln!(
            formatter,
            "{} [{}] ({}:{}): {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            formatter.default_styled_level(record.level()),
            record.target(),
            record.line().unwrap_or(0),
            record.args()
        )
    });

    builder
}
