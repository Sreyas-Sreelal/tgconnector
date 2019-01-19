#[macro_use]
extern crate samp_sdk;
extern crate minihttp;
extern crate serde_json;
extern crate serde;
extern crate encoding;

#[macro_use]
extern crate serde_derive;

#[macro_use]
mod macros;

mod types;
mod http;
mod plugin;
mod natives;
mod api;
mod functions;
mod internals;
mod callbacks;
mod encode;

use plugin::TgConnector;

new_plugin!(TgConnector with process_tick);