use api::BOT;
use encode::encode_replace;
use functions::*;
use internals::create_bot;
use samp_sdk::amx::AmxResult;
use samp_sdk::amx::AMX;
use samp_sdk::types::Cell;

impl super::TgConnector {
    pub fn bot_connect(&mut self, _amx: &AMX, token: String) -> AmxResult<Cell> {
        let api = BOT::new(token);
        create_bot(self, api)
    }

    pub fn bot_connect_from_env(&mut self, _amx: &AMX, variable: String) -> AmxResult<Cell> {
        let token = std::env::var_os(&variable);
        if token == None {
            log!(
                "**[TGConnector] Error environment variable {:?} is not set",
                variable
            );
            Ok(-1)
        } else {
            let token = token.unwrap().into_string().unwrap();
            let api = BOT::new(token);
            create_bot(self, api)
        }
    }

    pub fn bot_send_message(
        &mut self,
        _amx: &AMX,
        botid: usize,
        chatid: String,
        text: String,
        reply_id: i32,
        parse_mode: i32,
        callback: String,
    ) -> AmxResult<Cell> {
        let reply: Option<i32>;
        if reply_id == -1 {
            reply = None;
        } else {
            reply = Some(reply_id);
        }

        let parsemode: Option<&str> = match parse_mode {
            0 => Some("HTML"),
            1 => Some("markdown"),
            _ => None,
        };

        let callback: Option<String> = match callback.is_empty() {
            true => None,
            false => Some(callback),
        };

        if !self.bots.contains_key(&botid) {
            log!("**[TGConnector] Error Invalid bot id {} passed", botid);
            Ok(0)
        } else {
            let send_message_obj = SendMessage {
                chat_id: chatid,
                text: text,
                reply_to_message_id: reply,
                parse_mode: parsemode,
            };
            self.bots[&botid].send_message(send_message_obj, callback);
            Ok(1)
        }
    }

    pub fn bot_delete_message(
        &mut self,
        _amx: &AMX,
        botid: usize,
        chatid: String,
        messageid: i32,
    ) -> AmxResult<Cell> {
        if !self.bots.contains_key(&botid) {
            log!("**[TGConnector] Error Invalid bot id {} passed", botid);
            Ok(0)
        } else {
            let delete_message_obj = DeleteMessage {
                chat_id: chatid,
                message_id: messageid,
            };
            self.bots[&botid].delete_message(delete_message_obj);
            Ok(1)
        }
    }

    pub fn bot_edit_message(
        &mut self,
        _amx: &AMX,
        botid: usize,
        chatid: String,
        messageid: i32,
        text: String,
        parse_mode: i32,
    ) -> AmxResult<Cell> {
        let parsemode: Option<&str> = match parse_mode {
            0 => Some("HTML"),
            1 => Some("markdown"),
            _ => None,
        };

        if !self.bots.contains_key(&botid) {
            log!("**[TGConnector] Error Invalid bot id {} passed", botid);
            Ok(0)
        } else {
            let edit_message_obj = EditMessageText {
                chat_id: chatid,
                text: text,
                message_id: messageid,
                parse_mode: parsemode,
            };
            self.bots[&botid].edit_message(edit_message_obj);
            Ok(1)
        }
    }

    pub fn cache_get_message(
        &mut self,
        _amx: &AMX,
        dest: &mut Cell,
        size: usize,
    ) -> AmxResult<Cell> {
        let cache_list = &self.telegram_messages;
        cache_get!(cache_list,dest,size)
    }

    pub fn cache_get_username(
        &mut self,
        _amx: &AMX,
        dest: &mut Cell,
        size: usize,
    ) -> AmxResult<Cell> {
        let cache_list = &self.telegram_username;
        cache_get!(cache_list,dest,size)
    }

    pub fn cache_get_user_first_name(
        &mut self,
        _amx: &AMX,
        dest: &mut Cell,
        size: usize,
    ) -> AmxResult<Cell> {
        let cache_list = &self.telegram_firstname;
        cache_get!(cache_list,dest,size)
    }

    pub fn cache_get_user_last_name(
        &mut self,
        _amx: &AMX,
        dest: &mut Cell,
        size: usize,
    ) -> AmxResult<Cell> {
        let cache_list = &self.telegram_lastname;
        cache_get!(cache_list,dest,size)
    }

    pub fn cache_get_chatid(
        &mut self,
        _amx: &AMX,
        dest: &mut Cell,
        size: usize,
    ) -> AmxResult<Cell> {
        let cache_list = &self.telegram_chatid;
        cache_get!(cache_list,dest,size)
    }

    pub fn cache_get_chatname(
        &mut self,
        _amx: &AMX,
        dest: &mut Cell,
        size: usize,
    ) -> AmxResult<Cell> {
        let cache_list = &self.telegram_chatname;
        cache_get!(cache_list,dest,size)
    }

    pub fn cache_get_chattype(
        &mut self,
        _amx: &AMX,
        dest: &mut Cell,
        size: usize,
    ) -> AmxResult<Cell> {
        let cache_list = &self.telegram_chattype;
        cache_get!(cache_list,dest,size)
    }

    pub fn get_user_status(
        &mut self,
        _amx: &AMX,
        botid: usize,
        userid: i32,
        chatid: String,
    ) -> AmxResult<Cell> {
        if !self.bots.contains_key(&botid) {
            log!("**[TGConnector] Error Invalid bot id {} passed", botid);
            Ok(0)
        } else {
            let getchatmember = GetChatMember {
                user_id: userid,
                chat_id: chatid,
            };
            let chatmember = self.bots[&botid].get_chat_member(getchatmember);

            if chatmember.is_none() {
                Ok(0)
            } else {
                let chatmember = chatmember.unwrap();

                match chatmember.status.as_ref() {
                    "creator" => Ok(1),
                    "adminstrator" => Ok(2),
                    "member" => Ok(3),
                    "restricted" => Ok(4),
                    "left" => Ok(5),
                    "kicked" => Ok(6),
                    _ => Ok(0),
                }
            }
        }
    }

    pub fn get_username_from_id(
        &mut self,
        _amx: &AMX,
        botid: usize,
        userid: i32,
        chatid: String,
        dest: &mut Cell,
        size: usize,
    ) -> AmxResult<Cell> {
        if !self.bots.contains_key(&botid) {
            log!("**[TGConnector] Error Invalid bot id {} passed", botid);
            Ok(0)
        } else {
            let getchatmember = GetChatMember {
                user_id: userid,
                chat_id: chatid,
            };
            let chatmember = self.bots[&botid].get_chat_member(getchatmember);

            if chatmember.is_none() {
                Ok(0)
            } else {
                let chatmember = chatmember.unwrap();
                let username = &chatmember.user.username;
                if *username == None {
                    Ok(0)
                } else {
                    match encode_replace(username.as_ref().unwrap()) {
                        Ok(encoded) => {
                            set_string!(encoded, dest, size);
                            Ok(1)
                        }
                        Err(err) => {
                            log!("**[TGConnector][get_username_from_id] Failed encoding {:?} \n {:?}",username.as_ref().unwrap(),err);
                            Ok(0)
                        }
                    }
                }
            }
        }
    }

    pub fn get_display_name_from_id(
        &mut self,
        _amx: &AMX,
        botid: usize,
        userid: i32,
        chatid: String,
        dest: &mut Cell,
        size: usize,
    ) -> AmxResult<Cell> {
        if !self.bots.contains_key(&botid) {
            log!("**[TGConnector] Error Invalid bot id {} passed", botid);
            Ok(0)
        } else {
            let getchatmember = GetChatMember {
                user_id: userid,
                chat_id: chatid,
            };
            let chatmember = self.bots[&botid].get_chat_member(getchatmember);

            if chatmember.is_none() {
                Ok(0)
            } else {
                let chatmember = chatmember.unwrap();
                let displayname = match &chatmember.user.last_name {
                    None => chatmember.user.first_name,
                    Some(lastname) => chatmember.user.first_name + " " + lastname,
                };

                match encode_replace(&displayname) {
                    Ok(encoded) => {
                        set_string!(encoded, dest, size);
                        Ok(1)
                    }
                    Err(err) => {
                        log!("**[TGConnector][get_display_name_from_id] Failed encoding {:?} \n {:?}",displayname,err);
                        Ok(0)
                    }
                }
            }
        }
    }

    pub fn get_chat_members_count(
        &mut self,
        _amx: &AMX,
        botid: usize,
        chatid: String,
    ) -> AmxResult<Cell> {
        if !self.bots.contains_key(&botid) {
            log!("**[TGConnector] Error Invalid bot id {} passed", botid);
            Ok(-1)
        } else {
            let getchatmembercount = GetChatMembersCount { chat_id: chatid };
            match self.bots[&botid].get_chat_members_count(getchatmembercount) {
                None => Ok(-1),
                Some(count) => Ok(count),
            }
        }
    }

    pub fn get_chat_title(
        &mut self,
        _amx: &AMX,
        botid: usize,
        chatid: String,
        title: &mut Cell,
        size: usize,
    ) -> AmxResult<Cell> {
        if !self.bots.contains_key(&botid) {
            log!("**[TGConnector] Error Invalid bot id {} passed", botid);
            Ok(0)
        } else {
            let getchat = GetChat { chat_id: chatid };

            let chat = self.bots[&botid].get_chat(getchat);
            if chat.is_none() {
                Ok(0)
            } else {
                match chat.unwrap().title {
                    Some(chat_title) => match encode_replace(&chat_title) {
                        Ok(encoded) => {
                            set_string!(encoded, title, size);
                            Ok(1)
                        }
                        Err(err) => {
                            log!(
                                "**[TGConnector][get_chat_title] Failed encoding {:?} \n {:?}",
                                chat_title,
                                err
                            );
                            Ok(0)
                        }
                    },
                    None => Ok(0),
                }
            }
        }
    }

    pub fn get_chat_description(
        &mut self,
        _amx: &AMX,
        botid: usize,
        chatid: String,
        description: &mut Cell,
        size: usize,
    ) -> AmxResult<Cell> {
        if !self.bots.contains_key(&botid) {
            log!("**[TGConnector] Error Invalid bot id {} passed", botid);
            Ok(0)
        } else {
            let getchat = GetChat { chat_id: chatid };

            let chat = self.bots[&botid].get_chat(getchat);
            if chat.is_none() {
                Ok(0)
            } else {
                match chat.unwrap().description {
                    Some(chat_description) => match encode_replace(&chat_description) {
                        Ok(encoded) => {
                            set_string!(encoded, description, size);
                            Ok(1)
                        }
                        Err(err) => {
                            log!("**[TGConnector][get_chat_description] Failed encoding {:?} \n {:?}",chat_description,err);
                            Ok(0)
                        }
                    },
                    None => Ok(0),
                }
            }
        }
    }
}
