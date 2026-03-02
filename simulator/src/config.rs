use std::{collections::HashSet, str::FromStr};

use thiserror::Error;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Config {
    debug: HashSet<DebugFlag>,
    fullscreen: bool,
    theme: DisplayTheme,
    battery_capacity: f64,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum DebugFlag {
    TextBuffer,
}

impl FromStr for DebugFlag {
    type Err = EnumParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "text-buffer" => Self::TextBuffer,
            _ => return Err(EnumParseError { name: "debug flag", value: s.to_string() }),
        })
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub enum DisplayTheme {
    #[default]
    Dark,
    Light,
}

impl FromStr for DisplayTheme {
    type Err = EnumParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "default" => Self::default(),
            "dark" => Self::Dark,
            "light" => Self::Light,
            _ => return Err(EnumParseError { name: "theme", value: s.to_string() }),
        })
    }
}

#[derive(Debug, Error)]
#[error("Unknown {name} {name:?}")]
pub struct EnumParseError {
    name: &'static str,
    value: String,
}
