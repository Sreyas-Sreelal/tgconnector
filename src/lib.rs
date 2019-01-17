#[macro_use]
extern crate samp_sdk;
extern crate minihttp;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod types;
mod http;
mod plugin;
mod natives;
mod api;
mod functions;

use plugin::TgConnector;

new_plugin!(TgConnector with process_tick);