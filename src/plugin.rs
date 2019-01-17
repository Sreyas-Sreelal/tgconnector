use samp_sdk::consts::*;
use samp_sdk::types::Cell;
use samp_sdk::amx::AMX;
use api::BOT;
use internals;

define_native!(bot_connect,token:String);
define_native!(bot_connect_from_env,variable:String);
define_native!(bot_send_message,botid:usize,chatid:String,text:String,reply_id:i32,parse_mode:i32);
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
		internals::update_process(self);
		
		internals::clear_caches(&mut self.telegram_chatname);
		internals::clear_caches(&mut self.telegram_messages);
		internals::clear_caches(&mut self.telegram_username);
		internals::clear_caches(&mut self.telegram_chattype);
		internals::clear_caches(&mut self.telegram_chatid);
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