#[macro_use]
extern crate samp_sdk;

mod plugin;
mod natives;

use plugin::TgConnector;

new_plugin!(TgConnector with process_tick);
