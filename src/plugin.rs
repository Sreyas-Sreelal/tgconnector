use crate::api::BOT;
use crate::internals;
use samp_sdk::amx::{AmxResult, AMX};
use samp_sdk::consts::*;
use samp_sdk::types::Cell;
use samp_sdk::{define_native, expand_args, get_string, log, natives};

use std::collections::{HashMap, LinkedList};

define_native!(bot_connect, token: String);

define_native!(bot_connect_from_env, variable: String);
define_native!(get_bot_user_id, botid: usize);

define_native!(
    bot_send_message,
    botid: usize,
    chatid: String,
    text: String,
    reply_id: i32,
    parse_mode: i32,
    callback: String
);

define_native!(
	cache_get_message,
	dest:ref Cell,
	size:usize
);

define_native!(
	cache_get_username,
	dest:ref Cell,
	size:usize
);

define_native!(
	cache_get_user_first_name,
	dest:ref Cell,
	size:usize
);
define_native!(
	cache_get_user_last_name,
	dest:ref Cell,
	size:usize
);
define_native!(
	cache_get_chatid,
	dest:ref Cell,
	size:usize
);
define_native!(
	cache_get_chattype,
	dest:ref Cell,
	size:usize
);
define_native!(
	cache_get_chatname,
	dest:ref Cell,
	size:usize
);
define_native!(
    bot_delete_message,
    botid: usize,
    chatid: String,
    messageid: i32
);

define_native!(
    bot_edit_message,
    botid: usize,
    chatid: String,
    messageid: i32,
    text: String,
    parse_mode: i32
);

define_native!(get_user_status, botid: usize, userid: i32, chatid: String);

define_native!(
	get_username_from_id,
	botid:usize,
	userid:i32,
	chatid:String,
	dest:ref Cell,
	size:usize
);

define_native!(
	get_display_name_from_id,
	botid:usize,
	userid:i32,
	chatid:String,
	dest:ref Cell,
	size:usize
);

define_native!(get_chat_members_count, botid: usize, chatid: String);

define_native!(
	get_chat_title,
	botid:usize,
	chatid:String,
	title:ref Cell,
	size:usize
);

define_native!(
	get_chat_description,
	botid:usize,
	chatid:String,
	description:ref Cell,
	size:usize
);

pub struct TgConnector {
    plugin_version: i32,
    pub amx_list: Vec<usize>,
    pub bots: HashMap<usize, BOT>,
    pub bot_context_id: usize,
    pub telegram_messages: LinkedList<String>,
    pub telegram_username: LinkedList<String>,
    pub telegram_firstname: LinkedList<String>,
    pub telegram_lastname: LinkedList<String>,
    pub telegram_chatname: LinkedList<String>,
    pub telegram_chatid: LinkedList<String>,
    pub telegram_chattype: LinkedList<String>,
}

impl TgConnector {
    pub fn load(&self) -> bool {
        log!(
            "
   ###############################################################
   #                      TGConnector                            #
   #                        v0.1.0 Loaded!!                      #
   #   Found any bugs? Report it here:                           #
   #       https://github.com/Sreyas-Sreelal/tgconnector/issues  #
   #                                                             #
   ###############################################################
			"
        );
        return true;
    }

    pub fn unload(&self) {
        log!("**TGConnector v0.1.0 Unloaded!");
    }

    pub fn amx_load(&mut self, amx: &mut AMX) -> Cell {
        self.amx_list.push(amx.amx as usize);

        let natives = natives! {
            "TGConnect" => bot_connect,
            "TGConnectFromEnv" => bot_connect_from_env,
            "TGSendMessage" => bot_send_message,
            "TGCacheGetMessage" => cache_get_message,
            "TGCacheGetUserName" => cache_get_username,
            "TGCacheGetChatId" => cache_get_chatid,
            "TGCacheGetChatType" => cache_get_chattype,
            "TGCacheGetChatName" => cache_get_chatname,
            "TGCacheGetUserFirstName" => cache_get_user_first_name,
            "TGCacheGetUserLastName" => cache_get_user_last_name,
            "TGDeleteMessage" => bot_delete_message,
            "TGEditMessage" => bot_edit_message,
            "TGGetUserChatStatus" => get_user_status,
            "TGGetUserNameFromId" => get_username_from_id,
            "TGGetDisplayNameFromId" => get_display_name_from_id,
            "TGGetChatMembersCount" => get_chat_members_count,
            "TGGetChatTitle" => get_chat_title,
            "TGGetChatDescription" => get_chat_description,
            "TGGetBotUserId" => get_bot_user_id
        };

        match amx.register(&natives) {
            Ok(_) => log!("Natives are successful loaded"),
            Err(err) => log!("Whoops, there is an error {:?}", err),
        }

        let get_version: AmxResult<&mut i32> = amx.find_pubvar("_tgconnector_version");

        match get_version {
            Ok(version) => {
                if *version != self.plugin_version {
                    log!("**[TGConnector] Warning plugin and include version doesnot match : Include {:?} Plugin {:?}",version,self.plugin_version);
                }
            }
            Err(err) => log!(
                "**[TGConnector] Failed to retrive include version Reason:{:?}",
                err
            ),
        }

        AMX_ERR_NONE
    }

    pub fn amx_unload(&mut self, amx: &mut AMX) -> Cell {
        let raw = amx.amx as usize;
        let index = self.amx_list.iter().position(|x| *x == raw).unwrap();
        self.amx_list.remove(index);
        AMX_ERR_NONE
    }

    pub fn process_tick(&mut self) {
        internals::update_process(self);
        internals::on_send_message_process(self);

        internals::clear_caches(&mut self.telegram_chatname);
        internals::clear_caches(&mut self.telegram_messages);
        internals::clear_caches(&mut self.telegram_username);
        internals::clear_caches(&mut self.telegram_firstname);
        internals::clear_caches(&mut self.telegram_lastname);
        internals::clear_caches(&mut self.telegram_chattype);
        internals::clear_caches(&mut self.telegram_chatid);
    }
}

impl Default for TgConnector {
    fn default() -> Self {
        TgConnector {
            plugin_version: 10,
            amx_list: Vec::new(),
            bots: HashMap::new(),
            bot_context_id: 0,
            telegram_messages: LinkedList::new(),
            telegram_username: LinkedList::new(),
            telegram_chatname: LinkedList::new(),
            telegram_chatid: LinkedList::new(),
            telegram_firstname: LinkedList::new(),
            telegram_lastname: LinkedList::new(),
            telegram_chattype: LinkedList::new(),
        }
    }
}
