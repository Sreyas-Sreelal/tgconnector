use crate::api::BOT;
use crate::encode::encode_replace;
use crate::internals::create_bot;
use crate::methods::*;
use log::error;
use samp::native;
use samp::prelude::*;

impl super::TgConnector {
    #[native(name = "TGConnect")]
    pub fn bot_connect(
        &mut self,
        _amx: &Amx,
        token: AmxString,
        thread_count: i32,
    ) -> AmxResult<i32> {
        let api = BOT::new(token.to_string(), thread_count);
        create_bot(self, api)
    }

    #[native(name = "TGConnectFromEnv")]
    pub fn bot_connect_from_env(
        &mut self,
        _amx: &Amx,
        variable: AmxString,
        thread_count: i32,
    ) -> AmxResult<i32> {
        let variable = variable.to_string();
        let token = std::env::var_os(&variable);

        if token == None {
            error!("Environment variable {:?} is not set", variable);
            return Ok(-1);
        }

        let token = token.unwrap().into_string().unwrap();
        let api = BOT::new(token, thread_count);

        create_bot(self, api)
    }

    #[native(name = "TGSendMessage")]
    pub fn bot_send_message(
        &self,
        _amx: &Amx,
        botid: usize,
        chatid: AmxString,
        text: AmxString,
        reply_id: i32,
        parse_mode: i32,
        callback: AmxString,
    ) -> AmxResult<i32> {
        if !self.bots.contains_key(&botid) {
            error!("Invalid bot id {} passed", botid);
            return Ok(0);
        }

        let reply = if reply_id == -1 { None } else { Some(reply_id) };
        let callback = callback.to_string();

        let parsemode: Option<&str> = match parse_mode {
            0 => Some("HTML"),
            1 => Some("markdown"),
            _ => None,
        };

        let callback = if callback.is_empty() {
            None
        } else {
            Some(callback)
        };

        let send_message_obj = SendMessage {
            chat_id: chatid.to_string(),
            text: text.to_string(),
            reply_to_message_id: reply,
            parse_mode: parsemode,
        };

        self.bots[&botid].send_message(send_message_obj, callback);
        Ok(1)
    }

    #[native(name = "TGDeleteMessage")]
    pub fn bot_delete_message(
        &self,
        _amx: &Amx,
        botid: usize,
        chatid: AmxString,
        messageid: i32,
    ) -> AmxResult<i32> {
        if !self.bots.contains_key(&botid) {
            error!("Invalid bot id {} passed", botid);
            return Ok(0);
        }

        let delete_message_obj = DeleteMessage {
            chat_id: chatid.to_string(),
            message_id: messageid,
        };

        self.bots[&botid].delete_message(delete_message_obj);
        Ok(1)
    }

    #[native(name = "TGEditMessage")]
    pub fn bot_edit_message(
        &self,
        _amx: &Amx,
        botid: usize,
        chatid: AmxString,
        messageid: i32,
        text: AmxString,
        parse_mode: i32,
    ) -> AmxResult<i32> {
        let parsemode: Option<&str> = match parse_mode {
            0 => Some("HTML"),
            1 => Some("markdown"),
            _ => None,
        };

        if !self.bots.contains_key(&botid) {
            error!("Error Invalid bot id {} passed", botid);
            return Ok(0);
        }

        let edit_message_obj = EditMessageText {
            chat_id: chatid.to_string(),
            text: text.to_string(),
            message_id: messageid,
            parse_mode: parsemode,
        };

        self.bots[&botid].edit_message(edit_message_obj);
        Ok(1)
    }

    #[native(name = "TGGetBotUserId")]
    pub fn get_bot_user_id(&self, _amx: &Amx, botid: usize) -> AmxResult<i32> {
        if !self.bots.contains_key(&botid) {
            error!("Invalid bot id {} passed", botid);
            return Ok(-1);
        }
        Ok(self.bots[&botid].user_id)
    }

    #[native(name = "TGCacheGetMessage")]
    pub fn cache_get_message(
        &self,
        _amx: &Amx,
        dest: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<i32> {
        let cache_list = &self.telegram_messages;
        cache_get!(cache_list, dest, size)
    }

    #[native(name = "TGCacheGetUserName")]
    pub fn cache_get_username(
        &self,
        _amx: &Amx,
        dest: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<i32> {
        let cache_list = &self.telegram_username;
        cache_get!(cache_list, dest, size)
    }

    #[native(name = "TGCacheGetUserFirstName")]
    pub fn cache_get_user_first_name(
        &self,
        _amx: &Amx,
        dest: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<i32> {
        let cache_list = &self.telegram_firstname;
        cache_get!(cache_list, dest, size)
    }

    #[native(name = "TGCacheGetUserLastName")]
    pub fn cache_get_user_last_name(
        &self,
        _amx: &Amx,
        dest: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<i32> {
        let cache_list = &self.telegram_lastname;
        cache_get!(cache_list, dest, size)
    }

    #[native(name = "TGCacheGetChatId")]
    pub fn cache_get_chatid(&self, _amx: &Amx, dest: UnsizedBuffer, size: usize) -> AmxResult<i32> {
        let cache_list = &self.telegram_chatid;
        cache_get!(cache_list, dest, size)
    }

    #[native(name = "TGCacheGetChatName")]
    pub fn cache_get_chatname(
        &self,
        _amx: &Amx,
        dest: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<i32> {
        let cache_list = &self.telegram_chatname;
        cache_get!(cache_list, dest, size)
    }

    #[native(name = "TGCacheGetChatType")]
    pub fn cache_get_chattype(
        &self,
        _amx: &Amx,
        dest: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<i32> {
        let cache_list = &self.telegram_chattype;
        cache_get!(cache_list, dest, size)
    }

    #[native(name = "TGGetUserChatStatus")]
    pub fn get_user_status(
        &self,
        _amx: &Amx,
        botid: usize,
        userid: i32,
        chatid: AmxString,
    ) -> AmxResult<i32> {
        if !self.bots.contains_key(&botid) {
            error!("**[TGConnector] Error Invalid bot id {} passed", botid);
            return Ok(0);
        }

        let getchatmember = GetChatMember {
            user_id: userid,
            chat_id: chatid.to_string(),
        };
        let chatmember = self.bots[&botid].get_chat_member(getchatmember);
        if chatmember.is_none() {
            return Ok(0);
        }

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

    #[native(name = "TGGetUserNameFromId")]
    pub fn get_username_from_id(
        &self,
        _amx: &Amx,
        botid: usize,
        userid: i32,
        chatid: AmxString,
        dest: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<i32> {
        if !self.bots.contains_key(&botid) {
            error!("Invalid bot id {} passed", botid);
            return Ok(0);
        }

        let getchatmember = GetChatMember {
            user_id: userid,
            chat_id: chatid.to_string(),
        };
        let chatmember = self.bots[&botid].get_chat_member(getchatmember);

        if chatmember.is_none() {
            return Ok(0);
        }

        let chatmember = chatmember.unwrap();
        let username = &chatmember.user.username;
        if *username == None {
            return Ok(0);
        }

        match encode_replace(username.as_ref().unwrap()) {
            Ok(encoded) => {
                let mut dest = dest.into_sized_buffer(size);
                let _ = samp::cell::string::put_in_buffer(&mut dest, &encoded);
                Ok(1)
            }
            Err(err) => {
                error!(
                    "[get_username_from_id] Failed encoding {:?} \n {:?}",
                    username.as_ref().unwrap(),
                    err
                );
                Ok(0)
            }
        }
    }

    #[native(name = "TGGetDisplayNameFromId")]
    pub fn get_display_name_from_id(
        &self,
        _amx: &Amx,
        botid: usize,
        userid: i32,
        chatid: AmxString,
        dest: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<i32> {
        if !self.bots.contains_key(&botid) {
            error!("Invalid bot id {} passed", botid);
            return Ok(0);
        }

        let getchatmember = GetChatMember {
            user_id: userid,
            chat_id: chatid.to_string(),
        };

        let chatmember = self.bots[&botid].get_chat_member(getchatmember);
        if chatmember.is_none() {
            return Ok(0);
        }

        let chatmember = chatmember.unwrap();
        let displayname = match &chatmember.user.last_name {
            None => chatmember.user.first_name,
            Some(lastname) => chatmember.user.first_name + " " + lastname,
        };

        match encode_replace(&displayname) {
            Ok(encoded) => {
                let mut dest = dest.into_sized_buffer(size);
                let _ = samp::cell::string::put_in_buffer(&mut dest, &encoded);
                Ok(1)
            }
            Err(err) => {
                error!(
                    "get_display_name_from_id] Failed encoding {:?} \n {:?}",
                    displayname, err
                );
                Ok(0)
            }
        }
    }

    #[native(name = "TGGetChatMembersCount")]
    pub fn get_chat_members_count(
        &self,
        _amx: &Amx,
        botid: usize,
        chatid: AmxString,
    ) -> AmxResult<i32> {
        if !self.bots.contains_key(&botid) {
            error!("Invalid bot id {} passed", botid);
            return Ok(-1);
        }

        let getchatmembercount = GetChatMembersCount {
            chat_id: chatid.to_string(),
        };

        match self.bots[&botid].get_chat_members_count(getchatmembercount) {
            None => Ok(-1),
            Some(count) => Ok(count),
        }
    }

    #[native(name = "TGGetChatTitle")]
    pub fn get_chat_title(
        &self,
        _amx: &Amx,
        botid: usize,
        chatid: AmxString,
        dest: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<i32> {
        if !self.bots.contains_key(&botid) {
            error!("Invalid bot id {} passed", botid);
            return Ok(0);
        }

        let getchat = GetChat {
            chat_id: chatid.to_string(),
        };

        let chat = self.bots[&botid].get_chat(getchat);
        if chat.is_none() {
            return Ok(0);
        }

        if chat.as_ref().unwrap().title.is_none() {
            return Ok(0);
        }

        let chat_title = chat.unwrap().title.unwrap();
        match encode_replace(&chat_title) {
            Ok(encoded) => {
                let mut dest = dest.into_sized_buffer(size);
                let _ = samp::cell::string::put_in_buffer(&mut dest, &encoded);
                Ok(1)
            }
            Err(err) => {
                error!(
                    "[get_chat_title] Failed encoding {:?} \n {:?}",
                    chat_title, err
                );
                Ok(0)
            }
        }
    }

    #[native(name = "TGGetChatDescription")]
    pub fn get_chat_description(
        &self,
        _amx: &Amx,
        botid: usize,
        chatid: AmxString,
        dest: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<i32> {
        if !self.bots.contains_key(&botid) {
            error!("Invalid bot id {} passed", botid);
            return Ok(0);
        }

        let getchat = GetChat {
            chat_id: chatid.to_string(),
        };

        let chat = self.bots[&botid].get_chat(getchat);
        if chat.is_none() {
            return Ok(0);
        }

        if chat.as_ref().unwrap().description.is_none() {
            return Ok(0);
        }

        let chat_description = chat.unwrap().description.unwrap();
        match encode_replace(&chat_description) {
            Ok(encoded) => {
                let mut dest = dest.into_sized_buffer(size);
                let _ = samp::cell::string::put_in_buffer(&mut dest, &encoded);
                Ok(1)
            }
            Err(err) => {
                error!(
                    "[get_chat_description] Failed encoding {:?} \n {:?}",
                    chat_description, err
                );
                Ok(0)
            }
        }
    }
}
