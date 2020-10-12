#[macro_use]
extern crate log;
extern crate log4rs;

mod domain;
mod parser;
mod error;
mod types;
mod parser_tests;
mod strip;
mod strip_tests;
mod auth;

pub const BASE_URL: &str = "https://www.opensubtitles.org";

