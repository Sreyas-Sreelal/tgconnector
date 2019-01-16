use samp_sdk::consts::*;
use samp_sdk::types::Cell;
use samp_sdk::amx::AMX;
use telegram::BOT;

define_native!(bot_connect,token:String);
define_native!(bot_connect_from_env,variable:String);

pub struct TgConnector{
	//plugin_version: i32,
	pub amx_list: Vec<usize>,
	pub bots: std::collections::HashMap<usize,BOT>,
	pub bot_context_id: usize,
}

impl TgConnector {
	pub fn load(&self) -> bool {
		log!("Plugin Loaded!");
		return true;
	}

	pub fn unload(&self) {
		log!("Plugin Unloaded!");
	}

	pub fn amx_load(&mut self, amx: &mut AMX) -> Cell {
		self.amx_list.push(amx.amx as usize);
		let natives = natives!{
			"TGConnect" => bot_connect,
			"TGConnectFromEnv" => bot_connect_from_env
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

	pub fn process_tick(&self) {
		for (id,bot) in &self.bots{
			for update in bot.update_reciever.as_ref().unwrap().try_iter(){
				let results = update.result.unwrap();

				for result in results{
					let message = result.message.text;
					if  message != None{
						let message = message.unwrap();
						for amx in &self.amx_list{
							let amx = AMX::new(*amx as *mut _);
							let mut executed;
							let botid = id.clone();
							match exec_public!(amx,"OnTGMessage";botid,message => string) {
								Ok(_) =>{
									executed = true;
								},
								Err(_err) =>{
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
		}
	}
}