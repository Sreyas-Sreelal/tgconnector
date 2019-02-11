use crate::http::{HttpMethod, HttpRequest};
use crate::methods::*;
use crate::types::*;
use samp_sdk::log;
use serde_json::{from_str, to_string};
use std::collections::VecDeque;
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct BOT {
    pub api_request_link: String,
    pub user_id: i32,
    pub update_reciever: Option<Receiver<Update>>,
    pub update_sender: Option<Sender<Update>>,
    pub send_message_reciever: Option<Receiver<(Message, String)>>,
    pub send_message_sender: Option<Sender<(Message, String)>>,
}

impl BOT {
    pub fn new(bot_token: String) -> Self {
        let (update_sender, update_reciever) = channel();
        let (send_message_sender, send_message_reciever) = channel();

        BOT {
            api_request_link: String::from("https://api.telegram.org/bot") + &bot_token,
            user_id: -1,
            update_reciever: Some(update_reciever),
            update_sender: Some(update_sender),
            send_message_reciever: Some(send_message_reciever),
            send_message_sender: Some(send_message_sender),
        }
    }

    pub fn connect(&mut self) -> bool {
        let request = HttpRequest {
            url: format!("{}/getme", self.api_request_link),
            method: HttpMethod::Get,
            body: None,
        };

        match request.make_request() {
            Ok(response) => {
                let response: APIResponse<User> = from_str(&response).unwrap();

                if response.ok {
                    self.user_id = response.body.unwrap().id;
                    self.get_updates();
                    true
                } else {
                    log!("**[TGConnector] Error bot couldn't connect.{:?}", response);
                    false
                }
            }
            Err(err) => {
                log!("{:?}", err);
                false
            }
        }
    }

    fn get_updates(&self) {
        let update_move = self.update_sender.clone();
        let api_link = self.api_request_link.clone();

        let mut getupdate = GetUpdates { offset: -2 };

        std::thread::spawn(move || loop {
            let request = HttpRequest {
                url: format!("{}/getUpdates", api_link),
                method: HttpMethod::Post,
                body: Some(to_string(&getupdate).unwrap()),
            };

            match request.make_request() {
                Ok(response) => {
                    let update: APIResponse<VecDeque<Update>> = from_str(&response).unwrap();

                    let mut check_result: VecDeque<Update> = match update.body {
                        None => {
                            continue;
                        }
                        Some(check_result) => check_result,
                    };

                    let first_update = check_result.pop_front();

                    match first_update {
                        Some(result) => {
                            getupdate.offset = result.update_id + 1;
                            update_move.as_ref().unwrap().send(result).unwrap();
                        }

                        None => {
                            continue;
                        }
                    }
                }

                Err(err) => {
                    log!("{:?}", err);
                    continue;
                }
            }
        });
    }

    pub fn send_message(&self, send_message_obj: SendMessage, callback: Option<String>) {
        let send_message_move = self.send_message_sender.clone();
        let api_link = self.api_request_link.clone();

        std::thread::spawn(move || {
            let request = HttpRequest {
                url: format!("{}/sendmessage", api_link),
                method: HttpMethod::Post,
                body: Some(to_string(&send_message_obj).unwrap()),
            };

            match request.make_request() {
                Ok(response) => {
                    let response: APIResponse<Message> = from_str(&response).unwrap();
                    if !response.ok {
                        log!("**[TGConnector] Error Couldn't send message.{:?}", response);
                    } else if callback != None {
                        let sender = send_message_move.as_ref().unwrap();
                        let send_data = (response.body.unwrap(), callback.unwrap());
                        sender.send(send_data).unwrap();
                    }
                }

                Err(err) => {
                    log!("{:?}", err);
                }
            }
        });
    }

    pub fn delete_message(&self, delete_message_obj: DeleteMessage) {
        let api_link = self.api_request_link.clone();

        std::thread::spawn(move || {
            let request = HttpRequest {
                url: format!("{}/deletemessage", api_link),
                method: HttpMethod::Post,
                body: Some(to_string(&delete_message_obj).unwrap()),
            };

            match request.make_request() {
                Ok(response) => {
                    let response: APIResponse<bool> = from_str(&response).unwrap();
                    if !response.ok {
                        log!(
                            "**[TGConnector] Error Message {:?} couldn't delete. {:?}",
                            delete_message_obj,
                            response
                        );
                    }
                }

                Err(err) => {
                    log!("{:?}", err);
                }
            }
        });
    }

    pub fn edit_message(&self, edit_message_obj: EditMessageText) {
        let api_link = self.api_request_link.clone();

        std::thread::spawn(move || {
            let request = HttpRequest {
                url: format!("{}/editmessagetext", api_link),
                method: HttpMethod::Post,
                body: Some(to_string(&edit_message_obj).unwrap()),
            };

            match request.make_request() {
                Ok(response) => {
                    let response: APIResponse<Message> = from_str(&response).unwrap();
                    if !response.ok {
                        log!(
                            "**[TGConnector] Error Message {:?} couldn't edit {:?}",
                            edit_message_obj,
                            response
                        );
                    }
                }

                Err(err) => {
                    log!("{:?}", err);
                }
            }
        });
    }

    pub fn get_chat_member(&self, getchatmember: GetChatMember) -> Option<ChatMember> {
        let request = HttpRequest {
            url: format!("{}/getchatmember", self.api_request_link),
            method: HttpMethod::Post,
            body: Some(to_string(&getchatmember).unwrap()),
        };

        match request.make_request() {
            Ok(response) => {
                let response: APIResponse<ChatMember> = from_str(&response).unwrap();
                if response.ok {
                    response.body
                } else {
                    log!("**[TGConnector] Error get_chat_member.{:?}", response);
                    None
                }
            }

            Err(err) => {
                log!("{:?}", err);
                None
            }
        }
    }

    pub fn get_chat_members_count(&self, getchatmemberscount: GetChatMembersCount) -> Option<i32> {
        let request = HttpRequest {
            url: format!("{}/getchatmemberscount", self.api_request_link),
            method: HttpMethod::Post,
            body: Some(to_string(&getchatmemberscount).unwrap()),
        };

        match request.make_request() {
            Ok(response) => {
                let response: APIResponse<i32> = from_str(&response).unwrap();
                if response.ok {
                    response.body
                } else {
                    log!(
                        "**[TGConnector] Error get_chat_members_count.{:?}",
                        response
                    );
                    None
                }
            }

            Err(err) => {
                log!("{:?}", err);
                None
            }
        }
    }

    pub fn get_chat(&self, getchat: GetChat) -> Option<Chat> {
        let request = HttpRequest {
            url: format!("{}/getchat", self.api_request_link),
            method: HttpMethod::Post,
            body: Some(to_string(&getchat).unwrap()),
        };

        match request.make_request() {
            Ok(response) => {
                let response: APIResponse<Chat> = from_str(&response).unwrap();
                if response.ok {
                    response.body
                } else {
                    log!("**[TGConnector] Error get_chat.{:?}", response);
                    None
                }
            }

            Err(err) => {
                log!("{:?}", err);
                None
            }
        }
    }
}
