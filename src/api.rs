use std::sync::mpsc::{Sender,Receiver,channel};
use types::*;
use http::{HttpRequest,HttpMethod};
use functions::*;

pub struct BOT {
	pub api_requset_link: String,
	pub update_reciever: Option<Receiver<APIResponse>>,
	pub update_sender: Option<Sender<APIResponse>>
}

impl BOT {
	pub fn new(bot_token:String) -> Self {
		let (update_sender,update_reciever) = channel();
		
		BOT {
			api_requset_link: String::from("https://api.telegram.org/bot") + &bot_token,
			update_reciever: Some(update_reciever),
			update_sender: Some(update_sender),
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
		let mut getupdate = GetUpdates {
				offset: -2,
			};
		let api_link = self.api_requset_link.clone();

		std::thread::spawn(move|| {
			loop {
				let request = HttpRequest {
					url: format!("{}/getUpdates",api_link),
					method: HttpMethod::Post,
					body: Some(serde_json::to_string(&getupdate).unwrap()),
				};
			
				match request.make_request() {
					Ok(update) => {
						
						let check_result = match update.result.clone() {
							None => {
								continue;
							}
							Some(check_result) => {
								check_result
							}
						};

						let last_update = check_result.last();

						match last_update {
							Some(result) => {
								getupdate.offset = result.update_id+1;
								update_move.as_ref().unwrap().send(update.clone()).unwrap();
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

	
	pub fn send_message(&self,id:String,text:String) {
		let api_link = self.api_requset_link.clone();

		let send_message = SendMessage {
				chat_id: id,
				text: text,
		};

		std::thread::spawn(move || {

			let request = HttpRequest {
					url: format!("{}/sendmessage",api_link),
					method: HttpMethod::Post,
					body: Some(serde_json::to_string(&send_message).unwrap()),
			};

			match request.make_request() {
				Ok(_response) => {
					//TODO
				},

				Err(err) => {
					log!("{:?}",err);
				}
			}
		});
	}
}

