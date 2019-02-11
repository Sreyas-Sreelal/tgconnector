#[macro_use]
mod macros;
use plugin::TgConnector;
mod api;
mod callbacks;
mod encode;
mod http;
mod internals;
mod methods;
mod natives;
mod plugin;
mod types;

use samp_sdk::new_plugin;

new_plugin!(TgConnector with process_tick);
