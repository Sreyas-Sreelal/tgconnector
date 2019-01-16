use samp_sdk::consts::*;
use samp_sdk::types::Cell;
use samp_sdk::amx::AMX;
use telegram::BOT;

define_native!(bot_connect,token:String);
define_native!(bot_connect_from_env,variable:String);
define_native!(bot_send_message,botid:usize,chatid:String,text:String);
define_native!(get_message,dest:ref Cell,size:usize);
define_native!(get_username,dest:ref Cell,size:usize);
define_native!(get_chatid,dest:ref Cell,size:usize);
define_native!(get_chattype,dest:ref Cell,size:usize);
define_native!(get_chatname,dest:ref Cell,size:usize);

pub struct TgConnector {
	//plugin_version: i32,
	pub amx_list: Vec<usize>,
	pub bots: std::collections::HashMap<usize,BOT>,
	pub bot_context_id: usize,
	pub telegram_messages: std::collections::LinkedList<String>,
	pub telegram_username: std::collections::LinkedList<String>,
	pub telegram_chatname: std::collections::LinkedList<String>,
	pub telegram_chatid: std::collections::LinkedList<String>,
	pub telegram_chattype: std::collections::LinkedList<String>,
}

impl TgConnector {
	pub fn load(&self) -> bool {
		log!("**[TGConnector] Loaded!");
		return true;
	}

	pub fn unload(&self) {
		log!("**[TGConnector] Unloaded!");
	}

	pub fn amx_load(&mut self, amx: &mut AMX) -> Cell {
		self.amx_list.push(amx.amx as usize);

		let natives = natives!{
			"TGConnect" => bot_connect,
			"TGConnectFromEnv" => bot_connect_from_env,
			"TGSendMessage" => bot_send_message,
			"TGGetMessage" => get_message,
			"TGGetUserName" => get_username,
			"TGGetChatId" => get_chatid,
			"TGGetChatType" => get_chattype,
			"TGGetChatName" => get_chatname
		};

		match amx.register(&natives) {
			Ok(_) => log!("Natives are successful loaded"),
			Err(err) => log!("Whoops, there is an error {:?}", err),
		}

		AMX_ERR_NONE
	}

	pub fn amx_unload(&mut self, amx: &mut AMX) -> Cell {
		let raw = amx.amx as usize;
		let index = self.amx_list.iter().position(|x| *x == raw).unwrap().clone();
		self.amx_list.remove(index);
		AMX_ERR_NONE
	}

	pub fn process_tick(&mut self) {
		
		if !self.telegram_messages.is_empty() {
			self.telegram_messages.clear();
		}

		if !self.telegram_chattype.is_empty() {
			self.telegram_chattype.clear();
		}

		if !self.telegram_chatname.is_empty() {	
			self.telegram_chatname.clear();
		}

		if !self.telegram_chatid.is_empty() {	
			self.telegram_chatid.clear();
		}

		if !self.telegram_username.is_empty() {	
			self.telegram_username.clear();
		}

		//log!("process tick");
		for (id,bot) in &self.bots {
			for update in bot.update_reciever.as_ref().unwrap().try_iter() {
				let results = update.result.unwrap();

				for result in results {
					let message = result.message.text;

					if  message != None {
						self.telegram_messages.push_front(message.clone().unwrap());
						
						for amx in &self.amx_list{
							let amx = AMX::new(*amx as *mut _);
							let mut executed;
							let botid = id.clone();
							let fromid = result.message.from.id.clone();

							self.telegram_chatid.push_front(result.message.chat.id.clone());
							
							let username = match result.message.from.username.clone() {
								Some(username) => Some(username),
								None => None
							};

							if username != None{
								self.telegram_username.push_front(username.unwrap());
							}

							let chatname:Option<String> = match result.message.chat.title.clone() {
								Some(chatname) => Some(chatname),
								None => None
							};

							if chatname != None{
								self.telegram_chatname.push_front(chatname.unwrap());
							}
							
							self.telegram_chattype.push_front(result.message.chat.chat_type.clone());

							match exec_public!(amx,"OnTGMessage";botid,fromid) {
								Ok(_) => {
									executed = true;
								},

								Err(_err) => {
									continue;
								}
							}

							
							if !executed {
								log!("**[TGConnector] Error executing callback OnTGMessage");
							}
						}
					}
				}
			}
			
		}

		
	}
}

impl Default for TgConnector {
	fn default() -> Self {
		TgConnector {
			//plugin_version: 1,
			amx_list: Vec::new(),
			bots: std::collections::HashMap::new(),
			bot_context_id: 0,
			telegram_messages: std::collections::LinkedList::new(),
			telegram_username: std::collections::LinkedList::new(),
			telegram_chatname: std::collections::LinkedList::new(),
			telegram_chatid: std::collections::LinkedList::new(),
			telegram_chattype: std::collections::LinkedList::new(),
		}
	}
}