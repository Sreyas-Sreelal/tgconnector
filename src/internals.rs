use callbacks;
use types::*;

pub fn clear_caches(cache: &mut std::collections::LinkedList<String>) {
	if !cache.is_empty() {
		cache.clear();
	}
}

pub fn update_process(plugin: &mut super::TgConnector) {
	for (id,bot) in &plugin.bots {
		for update in bot.update_reciever.as_ref().unwrap().try_iter() {			
			match get_update_type(&update) {
				UpdateType::Message => {
					let message = update.message.unwrap();
					let user = message.from.unwrap();

					plugin.telegram_firstname.push_front(user.first_name);
					plugin.telegram_messages.push_front(message.text.unwrap());
					plugin.telegram_chatid.push_front(message.chat.id);
					plugin.telegram_chattype.push_front(message.chat.chat_type);
					
					match user.last_name {
						Some(lastname) => {
							plugin.telegram_lastname.push_front(lastname);
						}
						None => {
						}
					};

					match user.username {
						Some(username) => {
							plugin.telegram_username.push_front(username);
						}
						None => {
						}
					};
					
					match message.chat.title {
						Some(chatname) => {
							plugin.telegram_chatname.push_front(chatname);
						}
						None => {
							
						}
					};                     
					
					callbacks::on_tg_message(plugin,id, user.id,message.message_id);	
				}

				UpdateType::ChannelPost => {
					let message = update.channel_post.unwrap();
					plugin.telegram_messages.push_front(message.text.unwrap());
					plugin.telegram_chatid.push_front(message.chat.id);

					match message.chat.title {
							Some(chatname) => {
								plugin.telegram_chatname.push_front(chatname);
							}
							None => {
								
							}
					};
					callbacks::on_tg_channel_post(plugin,id,message.message_id);
				}

				UpdateType::UserJoined => {
					let message = update.message.unwrap();
					let user = message.from.unwrap();

					plugin.telegram_firstname.push_front(user.first_name);
					
					match user.last_name {
						Some(lastname) => {
							plugin.telegram_lastname.push_front(lastname);
						}
						None => {
						}
					};

					match message.chat.title {
							Some(chatname) => {
								plugin.telegram_chatname.push_front(chatname);
							}
							None => {
								
							}
					};
					
					plugin.telegram_chatid.push_front(message.chat.id);

					for user in message.new_chat_members.unwrap() {
						match user.username {
							Some(username) => {
								plugin.telegram_username.push_front(username);
							}
							None => {
							}
						};
						callbacks::on_tg_user_joined(plugin,id,user.id);	
					}
				}

				UpdateType::UserLeft => {
					let message = update.message.unwrap();
					let user = message.from.unwrap();

					plugin.telegram_firstname.push_front(user.first_name);
					
					match user.last_name {
						Some(lastname) => {
							plugin.telegram_lastname.push_front(lastname);
						}
						None => {
						}
					};

					match message.chat.title {
							Some(chatname) => {
								plugin.telegram_chatname.push_front(chatname);
							}
							None => {
								
							}
					};
					
					plugin.telegram_chatid.push_front(message.chat.id);

					let user = message.left_chat_member.unwrap();
					match user.username {
						Some(username) => {
							plugin.telegram_username.push_front(username);
						}
						None => {
						}
					};
					callbacks::on_tg_user_left(plugin,id,user.id);	
					
				}

				UpdateType::UnknownUpdate => {
					continue;
				}
			}	
			
		}
	}
}

fn get_update_type(update:&Update) -> UpdateType{
	if update.message.is_some() {
		let message = update.message.as_ref().unwrap();
		if message.text.is_some() {
			UpdateType::Message
		} else if message.new_chat_members.is_some() {
			UpdateType::UserJoined
		} else if message.left_chat_member.is_some() {
			UpdateType::UserLeft
		} else {
			UpdateType::UnknownUpdate
		}
	} else if update.channel_post.is_some() {
		UpdateType::ChannelPost
	} else {
		UpdateType::UnknownUpdate
	}
}
pub fn on_send_message_process(plugin: &mut super::TgConnector) {
	for (id,bot) in &plugin.bots {
		for (message,callback) in bot.send_message_reciever.as_ref().unwrap().try_iter() {
			if message.text != None {
				plugin.telegram_messages.push_front(message.text.unwrap());
				plugin.telegram_chatid.push_front(message.chat.id);
				callbacks::on_tg_send_message(plugin,callback,id,message.message_id);
			}
		}
	}
}
