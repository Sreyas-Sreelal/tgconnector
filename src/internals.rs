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
					plugin.telegram_messages.push_front(update.message.text.unwrap());
					plugin.telegram_chatid.push_front(update.message.chat.id);
					plugin.telegram_chattype.push_front(update.message.chat.chat_type);
					plugin.telegram_firstname.push_front(update.message.from.first_name);
					
					match update.message.from.last_name {
						Some(lastname) => {
							plugin.telegram_lastname.push_front(lastname);
						}
						None => {
						}
					};

					match update.message.from.username {
						Some(username) => {
							plugin.telegram_username.push_front(username);
						}
						None => {
						}
					};
					
					match update.message.chat.title {
						Some(chatname) => {
							plugin.telegram_chatname.push_front(chatname);
						}
						None => {
							
						}
					};                     
					
					callbacks::on_tg_message(plugin,id, update.message.from.id,update.message.message_id);	
				}
				
				UpdateType::UserJoined => {
					plugin.telegram_firstname.push_front(update.message.from.first_name);
					
					match update.message.from.last_name {
						Some(lastname) => {
							plugin.telegram_lastname.push_front(lastname);
						}
						None => {
						}
					};

					match update.message.chat.title {
							Some(chatname) => {
								plugin.telegram_chatname.push_front(chatname);
							}
							None => {
								
							}
					};
					
					plugin.telegram_chatid.push_front(update.message.chat.id);

					for user in update.message.new_chat_members.unwrap() {
						match user.username {
							Some(username) => {
								plugin.telegram_username.push_front(username);
							}
							None => {
							}
						};
						callbacks::ong_tg_user_joined(plugin,id,user.id);	
					}
				}

				UpdateType::UserLeft => {
					plugin.telegram_firstname.push_front(update.message.from.first_name);
					
					match update.message.from.last_name {
						Some(lastname) => {
							plugin.telegram_lastname.push_front(lastname);
						}
						None => {
						}
					};

					match update.message.chat.title {
							Some(chatname) => {
								plugin.telegram_chatname.push_front(chatname);
							}
							None => {
								
							}
					};
					
					plugin.telegram_chatid.push_front(update.message.chat.id);

					let user = update.message.left_chat_member.unwrap();
					match user.username {
						Some(username) => {
							plugin.telegram_username.push_front(username);
						}
						None => {
						}
					};
					callbacks::ong_tg_user_left(plugin,id,user.id);	
					
				}

				UpdateType::UnknownUpdate => {
					continue;
				}
			}	
			
		}
	}
}

fn get_update_type(update:&Update) -> UpdateType{
	if update.message.text != None {
		UpdateType::Message
	} else if update.message.new_chat_members.is_some() {
		UpdateType::UserJoined
	} else if update.message.left_chat_member.is_some() {
		UpdateType::UserLeft
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
