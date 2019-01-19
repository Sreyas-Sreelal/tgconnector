use std::sync::mpsc::{Sender,Receiver,channel};
use types::*;
use http::{HttpRequest,HttpMethod};
use functions::*;

pub struct BOT {
	pub api_requset_link: String,
	pub update_reciever: Option<Receiver<Update>>,
	pub update_sender: Option<Sender<Update>>,
	pub send_message_reciever: Option<Receiver<(Message,String)>>,
	pub send_message_sender: Option<Sender<(Message,String)>>,
}

impl BOT {
	pub fn new(bot_token:String) -> Self {
		let (update_sender,update_reciever) = channel();
		let (send_message_sender,send_message_reciever) = channel();
		
		BOT {
			api_requset_link: String::from("https://api.telegram.org/bot") + &bot_token,
			update_reciever: Some(update_reciever),
			update_sender: Some(update_sender),
			send_message_reciever: Some(send_message_reciever),
			send_message_sender: Some(send_message_sender),
		}
	} 

	pub fn connect(&self) -> bool {

		let request = HttpRequest {
			url: format!("{}/getme",self.api_requset_link),
			method: HttpMethod::Get,
			body: None,
		};
			   
		match request.make_request() {
			Ok(response) => {
				let response:APIResponse<User> = serde_json::from_str(&response).unwrap();
				
				if response.ok {
					self.get_updates();
					true
				}else {
					log!("**[TGConnector] Error Invalid token is passed");
					false
				}
			}
			Err(err) => {
				log!("{:?}",err);
				false
			}
		}
	}
	
	fn get_updates(&self) {
		let update_move = self.update_sender.clone();
		let api_link = self.api_requset_link.clone();
		
		let mut getupdate = GetUpdates {
				offset: -2,
		};
		

		std::thread::spawn(move|| {
			loop {
				let request = HttpRequest {
					url: format!("{}/getUpdates",api_link),
					method: HttpMethod::Post,
					body: Some(serde_json::to_string(&getupdate).unwrap()),
				};
			
				match request.make_request() {
					Ok(response) => {
						let update:APIResponse<Vec<Update>> = serde_json::from_str(&response).unwrap();
						
						let check_result:&Vec<Update> = match &update.result {
							None => {
								continue;
							}
							Some(check_result) => {
								&check_result
							}
						};

						let first_update = &check_result.first();

						match first_update {
							Some(result) => {
								getupdate.offset = result.update_id+1;
								update_move.as_ref().unwrap().send(first_update.unwrap().clone()).unwrap();
							}

							None => {
								continue;
							}
						}
					}

					Err(err) => {
						log!("{:?}",err);
						continue;                       
					}
				}
			}
		});
	}

	pub fn send_message(&self,send_message_obj:SendMessage,callback:Option<String>) {
		let send_message_move = self.send_message_sender.clone();
		let api_link = self.api_requset_link.clone();

		std::thread::spawn(move || {

			let request = HttpRequest {
					url: format!("{}/sendmessage",api_link),
					method: HttpMethod::Post,
					body: Some(serde_json::to_string(&send_message_obj).unwrap()),
			};
			
			match request.make_request() {
				Ok(response) => {
					let response:APIResponse<Message> = serde_json::from_str(&response).unwrap();
					if callback != None && response.ok {
						send_message_move.as_ref().unwrap().send((response.result.unwrap(),callback.unwrap())).unwrap();
					}
				},

				Err(err) => {
					log!("{:?}",err);
				}
			}
		});
	}

	pub fn delete_message(&self,delete_message_obj:DeleteMessage) {
		let api_link = self.api_requset_link.clone();

		std::thread::spawn(move || {

			let request = HttpRequest {
					url: format!("{}/deletemessage",api_link),
					method: HttpMethod::Post,
					body: Some(serde_json::to_string(&delete_message_obj).unwrap()),
			};
			
			match request.make_request() {
				Ok(response) => {
					let response:APIResponse<Message> = serde_json::from_str(&response).unwrap();
					if !response.ok {
						log!("**[TGConnector] Error Message {:?} in chat {:?} couldn't delete (Check bot permissions!)",delete_message_obj.message_id,delete_message_obj.chat_id);
					}
				},

				Err(err) => {
					log!("{:?}",err);
				}
			}
		});
	}

	pub fn edit_message(&self,edit_message_obj:EditMessageText) {
		let api_link = self.api_requset_link.clone();

		std::thread::spawn(move || {

			let request = HttpRequest {
					url: format!("{}/editmessagetext",api_link),
					method: HttpMethod::Post,
					body: Some(serde_json::to_string(&edit_message_obj).unwrap()),
			};
			
			match request.make_request() {
				Ok(response) => {
					let response:APIResponse<Message> = serde_json::from_str(&response).unwrap();
					if !response.ok {
						log!("**[TGConnector] Error Message {:?} in chat {:?} couldn't edit (Check bot permissions!)",edit_message_obj.message_id,edit_message_obj.chat_id);
					}
				},

				Err(err) => {
					log!("{:?}",err);
				}
			}
		});
	}
}

