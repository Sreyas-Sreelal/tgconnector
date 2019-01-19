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
			let results = update.result.unwrap();
			for result in results {
				match get_update_type(result.clone()) {
					UpdateType::Message => {
						plugin.telegram_messages.push_front(result.message.text.unwrap());
						plugin.telegram_chatid.push_front(result.message.chat.id);
						plugin.telegram_chattype.push_front(result.message.chat.chat_type);
						
						match result.message.from.username {
							Some(username) => {
								plugin.telegram_username.push_front(username);
							}
							None => {
							}
						};
						
						match result.message.chat.title {
							Some(chatname) => {
								plugin.telegram_chatname.push_front(chatname);
							}
							None => {
								
							}
						};                     
						
						callbacks::on_tg_message(plugin,id.clone(), result.message.from.id.clone(),result.message.message_id.clone());	
					}
					
					UpdateType::UserJoined => {
						for user in result.message.new_chat_members.unwrap() {
							match user.username {
								Some(username) => {
									plugin.telegram_username.push_front(username);
								}
								None => {
								}
							};

							match result.message.chat.title.clone() {
								Some(chatname) => {
									plugin.telegram_chatname.push_front(chatname);
								}
								None => {
									
								}
							}; 

							plugin.telegram_chatid.push_front(result.message.chat.id.clone());
							callbacks::ong_tg_user_joined(plugin,id.clone(),user.id);	
						}
					}

					UpdateType::UserLeft => {

					}
				}	
			}
		}
	}
}

fn get_update_type(update:Update) -> UpdateType{
	if update.message.text != None {
		UpdateType::Message
	} else if update.message.new_chat_members.is_some() {
		UpdateType::UserJoined
	} else {
		UpdateType::UserLeft
	}

}
pub fn on_send_message_process(plugin: &mut super::TgConnector) {
	for (id,bot) in &plugin.bots {
		for (response,callback) in bot.send_message_reciever.as_ref().unwrap().try_iter() {
			let result = response.result.unwrap();

			if result.text != None {
				plugin.telegram_messages.push_front(result.text.unwrap());
				plugin.telegram_chatid.push_front(result.chat.id);
				callbacks::on_tg_send_message(plugin,callback,id.clone(),result.message_id);
			}
		}
	}
}
