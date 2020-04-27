use crate::http::{HttpMethod, HttpRequest};
use crate::methods::*;
use crate::types::*;
use log::error;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{from_str, to_string};
use std::collections::VecDeque;
use std::sync::mpsc::{channel, Receiver, Sender};
use threadpool::ThreadPool;

pub struct BOT {
    pub api_request_link: String,
    pub user_id: i32,
    pub update_reciever: Option<Receiver<Update>>,
    pub update_sender: Option<Sender<Update>>,
    pub send_message_reciever: Option<Receiver<(Message, String)>>,
    pub send_message_sender: Option<Sender<(Message, String)>>,
    pub pool: ThreadPool,
    pub proxy_url: Option<String>,
}

impl BOT {
    pub fn new(bot_token: String, thread_count: i32, proxy_url: Option<String>) -> Self {
        let (update_sender, update_reciever) = channel();
        let (send_message_sender, send_message_reciever) = channel();

        BOT {
            api_request_link: String::from("https://api.telegram.org/bot") + &bot_token,
            user_id: -1,
            update_reciever: Some(update_reciever),
            update_sender: Some(update_sender),
            send_message_reciever: Some(send_message_reciever),
            send_message_sender: Some(send_message_sender),
            pool: ThreadPool::new(thread_count as usize),
            proxy_url,
        }
    }

    pub fn connect(&mut self, proxy_url: Option<String>) -> bool {
        let request = HttpRequest {
            url: format!("{}/getme", self.api_request_link),
            method: HttpMethod::Get,
            body: None,
            proxy_url,
        };

        match request.make_request() {
            Ok(response) => {
                let response: APIResponse<User> = from_str(&response).unwrap();

                if response.ok {
                    self.user_id = response.body.unwrap().id;
                    self.get_updates();
                    true
                } else {
                    error!("Bot couldn't connect.{:?}", response);
                    false
                }
            }
            Err(err) => {
                error!("{:?}", err);
                false
            }
        }
    }

    fn get_updates(&self) {
        let update_move = self.update_sender.clone();
        let api_link = self.api_request_link.clone();
        let proxy_url = self.proxy_url.clone();
        let mut getupdate = GetUpdates { offset: -2 };

        self.pool.execute(move || loop {
            let update: Result<APIResponse<VecDeque<Update>>, String> =
                telegram_request("getUpdates", &api_link, &getupdate, &proxy_url);
            match update {
                Ok(update) => {
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
                    error!("{:?}", err);
                    continue;
                }
            }
        });
    }

    pub fn send_message(&self, send_message_obj: SendMessage, callback: Option<String>) {
        let send_message_move = self.send_message_sender.clone();
        let api_link = self.api_request_link.clone();
        let proxy_url = self.proxy_url.clone();

        self.pool.execute(move || {
            let response: Result<APIResponse<Message>, String> =
                telegram_request("sendmessage", &api_link, &send_message_obj, &proxy_url);
            match response {
                Ok(response) => {
                    if !response.ok {
                        error!("Couldn't send message.{:?}", response);
                    } else if callback != None {
                        let sender = send_message_move.as_ref().unwrap();
                        let send_data = (response.body.unwrap(), callback.unwrap());
                        sender.send(send_data).unwrap();
                    }
                }

                Err(err) => {
                    error!("{:?}", err);
                }
            }
        });
    }

    pub fn delete_message(&self, delete_message_obj: DeleteMessage) {
        let api_link = self.api_request_link.clone();
        let proxy_url = self.proxy_url.clone();

        self.pool.execute(move || {
            let response: Result<APIResponse<bool>, String> =
                telegram_request("deletemessage", &api_link, &delete_message_obj, &proxy_url);

            match response {
                Ok(response) => {
                    if !response.ok {
                        error!(
                            "Message {:?} couldn't delete. {:?}",
                            delete_message_obj, response
                        );
                    }
                }

                Err(err) => {
                    error!("{:?}", err);
                }
            }
        });
    }

    pub fn edit_message(&self, edit_message_obj: EditMessageText) {
        let api_link = self.api_request_link.clone();
        let proxy_url = self.proxy_url.clone();

        self.pool.execute(move || {
            let response: Result<APIResponse<Message>, String> =
                telegram_request("editmessagetext", &api_link, &edit_message_obj, &proxy_url);
            match response {
                Ok(response) => {
                    if !response.ok {
                        error!(
                            "Message {:?} couldn't edit {:?}",
                            edit_message_obj, response
                        );
                    }
                }

                Err(err) => {
                    error!("{:?}", err);
                }
            }
        });
    }

    pub fn get_chat_member(&self, getchatmember: GetChatMember) -> Option<ChatMember> {
        let response: Result<APIResponse<ChatMember>, String> = telegram_request(
            "getchatmember",
            &self.api_request_link,
            &getchatmember,
            &self.proxy_url,
        );

        match response {
            Ok(response) => {
                if response.ok {
                    response.body
                } else {
                    error!("get_chat_member.{:?}", response);
                    None
                }
            }

            Err(err) => {
                error!("{:?}", err);
                None
            }
        }
    }

    pub fn get_chat_members_count(&self, getchatmemberscount: GetChatMembersCount) -> Option<i32> {
        let response: Result<APIResponse<i32>, String> = telegram_request(
            "getchatmemberscount",
            &self.api_request_link,
            &getchatmemberscount,
            &self.proxy_url,
        );

        match response {
            Ok(response) => {
                if response.ok {
                    response.body
                } else {
                    error!("get_chat_members_count.{:?}", response);
                    None
                }
            }

            Err(err) => {
                error!("{:?}", err);
                None
            }
        }
    }

    pub fn get_chat(&self, getchat: GetChat) -> Option<Chat> {
        let response: Result<APIResponse<Chat>, String> =
            telegram_request("getchat", &self.api_request_link, &getchat, &self.proxy_url);

        match response {
            Ok(response) => {
                if response.ok {
                    response.body
                } else {
                    error!("get_chat.{:?}", response);
                    None
                }
            }

            Err(err) => {
                error!("{:?}", err);
                None
            }
        }
    }
}

fn telegram_request<T: DeserializeOwned, B: Serialize>(
    endpoint: &str,
    api_link: &str,
    body: B,
    proxy_url: &Option<String>,
) -> Result<APIResponse<T>, String> {
    let request = HttpRequest {
        url: format!("{}/{}", api_link, endpoint),
        method: HttpMethod::Post,
        body: Some(to_string(&body).unwrap()),
        proxy_url: proxy_url.clone(),
    };

    match request.make_request() {
        Ok(response) => Ok(from_str(&response).unwrap()),
        Err(err) => Err(err),
    }
}
