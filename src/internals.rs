use crate::api::Bot;
use crate::callbacks;
use crate::types::*;
use samp::prelude::*;
use std::collections::LinkedList;

pub fn clear_caches(cache: &mut LinkedList<String>) {
    if !cache.is_empty() {
        cache.clear();
    }
}

pub fn update_process(plugin: &mut super::TgConnector) {
    for (id, bot) in &plugin.bots {
        for update in bot.update_reciever.as_ref().unwrap().try_iter() {
            match get_update_type(&update) {
                UpdateType::Message => {
                    let message = update.message.unwrap();
                    let user = message.from.unwrap();

                    plugin.telegram_firstname.push_front(user.first_name);
                    plugin.telegram_messages.push_front(message.text.unwrap());
                    plugin.telegram_chatid.push_front(message.chat.id);
                    plugin.telegram_chattype.push_front(message.chat.chat_type);

                    if let Some(lastname) = user.last_name {
                        plugin.telegram_lastname.push_front(lastname);
                    }

                    if let Some(username) = user.username {
                        plugin.telegram_username.push_front(username);
                    }

                    if let Some(chatname) = message.chat.title {
                        plugin.telegram_chatname.push_front(chatname);
                    }

                    callbacks::on_tg_message(&plugin.amx_list, *id, user.id, message.message_id);
                }

                UpdateType::ChannelPost => {
                    let message = update.channel_post.unwrap();
                    plugin.telegram_messages.push_front(message.text.unwrap());
                    plugin.telegram_chatid.push_front(message.chat.id);

                    if let Some(chatname) = message.chat.title {
                        plugin.telegram_chatname.push_front(chatname);
                    }
                    callbacks::on_tg_channel_post(&plugin.amx_list, *id, message.message_id);
                }

                UpdateType::UserJoined => {
                    let message = update.message.unwrap();
                    let user = message.from.unwrap();

                    plugin.telegram_firstname.push_front(user.first_name);

                    if let Some(lastname) = user.last_name {
                        plugin.telegram_lastname.push_front(lastname);
                    }

                    if let Some(chatname) = message.chat.title {
                        plugin.telegram_chatname.push_front(chatname);
                    }

                    plugin.telegram_chatid.push_front(message.chat.id);

                    for user in message.new_chat_members.unwrap() {
                        if let Some(username) = user.username {
                            plugin.telegram_username.push_front(username);
                        }

                        callbacks::on_tg_user_joined(&plugin.amx_list, *id, user.id);
                    }
                }

                UpdateType::UserLeft => {
                    let message = update.message.unwrap();
                    let user = message.from.unwrap();

                    plugin.telegram_firstname.push_front(user.first_name);

                    if let Some(lastname) = user.last_name {
                        plugin.telegram_lastname.push_front(lastname);
                    }

                    if let Some(chatname) = message.chat.title {
                        plugin.telegram_chatname.push_front(chatname);
                    }

                    plugin.telegram_chatid.push_front(message.chat.id);

                    let user = message.left_chat_member.unwrap();

                    if let Some(username) = user.username {
                        plugin.telegram_username.push_front(username);
                    }

                    callbacks::on_tg_user_left(&plugin.amx_list, *id, user.id);
                }

                UpdateType::UnknownUpdate => {
                    continue;
                }
            }
        }
    }
}

fn get_update_type(update: &Update) -> UpdateType {
    if update.message.is_some() {
        let message = update.message.as_ref().unwrap();
        if message.text.is_some() {
            return UpdateType::Message;
        } else if message.new_chat_members.is_some() {
            return UpdateType::UserJoined;
        } else if message.left_chat_member.is_some() {
            return UpdateType::UserLeft;
        }
    } else if update.channel_post.is_some() {
        let post = update.channel_post.as_ref().unwrap();
        if post.text.is_some() {
            return UpdateType::ChannelPost;
        }
    }
    UpdateType::UnknownUpdate
}

pub fn on_send_message_process(plugin: &mut super::TgConnector) {
    for (id, bot) in &plugin.bots {
        for (message, callback) in bot.send_message_reciever.as_ref().unwrap().try_iter() {
            if message.text != None {
                plugin.telegram_messages.push_front(message.text.unwrap());
                plugin.telegram_chatid.push_front(message.chat.id);
                callbacks::on_tg_send_message(&plugin.amx_list, &callback, *id, message.message_id);
            }
        }
    }
}

pub fn create_bot(
    plugin: &mut super::TgConnector,
    mut api: Bot,
    proxy_url: Option<String>,
) -> AmxResult<i32> {
    if api.connect(proxy_url) {
        plugin.bots.insert(plugin.bot_context_id, api);
        plugin.bot_context_id += 1;
        Ok(plugin.bot_context_id as i32 - 1)
    } else {
        Ok(-1)
    }
}

pub fn get_parse_mode(numerical_code: i32) -> Option<&'static str> {
    match numerical_code {
        0 => Some("HTML"),
        1 => Some("markdown"),
        2 => Some("MarkdownV2"),
        _ => None,
    }
}
