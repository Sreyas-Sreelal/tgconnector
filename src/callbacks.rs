use samp_sdk::amx::AMX;

pub fn on_tg_message(plugin: &super::TgConnector,botid: usize,fromid:i32,message_id:i32) {
	for amx in &plugin.amx_list{
		let amx = AMX::new(*amx as *mut _);
		let mut executed;
	   
		match exec_public!(amx,"OnTGMessage";botid,fromid,message_id) {
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

pub fn on_tg_send_message(plugin: &super::TgConnector,name:String,botid: usize,message_id:i32) {
	for amx in &plugin.amx_list{
		let amx = AMX::new(*amx as *mut _);
		let mut executed;
	   
		match exec_public_with_name!(amx,name;botid,message_id) {
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