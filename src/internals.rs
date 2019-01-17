use callbacks;

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
				if result.message.text != None {
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
			}
		}
	}
}