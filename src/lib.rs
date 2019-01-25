#[macro_use]
extern crate samp_sdk;
extern crate encoding;
extern crate minihttp;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
mod macros;

mod api;
mod callbacks;
mod encode;
mod functions;
mod http;
mod internals;
mod natives;
mod plugin;
mod types;

use plugin::TgConnector;

new_plugin!(TgConnector with process_tick);
