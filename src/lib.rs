#[macro_use]
mod macros;
mod api;
mod callbacks;
mod encode;
mod http;
mod internals;
mod methods;
mod natives;
mod plugin;
mod types;

use crate::plugin::TgConnector;
use samp::initialize_plugin;

use std::collections::{HashMap, LinkedList};
initialize_plugin!(
    natives: [
        TgConnector::bot_connect,
        TgConnector::bot_connect_from_env,
        TgConnector::bot_send_message,
        TgConnector::bot_delete_message,
        TgConnector::bot_edit_message,
        TgConnector::get_bot_user_id,
        TgConnector::cache_get_message,
        TgConnector::cache_get_username,
        TgConnector::cache_get_user_first_name,
        TgConnector::cache_get_user_last_name,
        TgConnector::cache_get_chatid,
        TgConnector::cache_get_chatname,
        TgConnector::cache_get_chattype,
        TgConnector::get_user_status,
        TgConnector::get_username_from_id,
        TgConnector::get_display_name_from_id,
        TgConnector::get_chat_members_count,
        TgConnector::get_chat_title,
        TgConnector::get_chat_description
    ],
    {
        let samp_logger = samp::plugin::logger()
            .level(log::LevelFilter::Info);

        let _ = fern::Dispatch::new()
            .format(|callback, message, record| {
                callback.finish(format_args!("[TgConnector] [{}]: {}", record.level().to_string().to_lowercase(), message))
            })
            .chain(samp_logger)
            .apply();

        TgConnector {
            plugin_version: 20,
            amx_list: Vec::new(),
            bots: HashMap::new(),
            bot_context_id: 0,
            telegram_messages: LinkedList::new(),
            telegram_username: LinkedList::new(),
            telegram_chatname: LinkedList::new(),
            telegram_chatid: LinkedList::new(),
            telegram_firstname: LinkedList::new(),
            telegram_lastname: LinkedList::new(),
            telegram_chattype: LinkedList::new(),
        }
    }
);
