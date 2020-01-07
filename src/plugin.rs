use crate::api::BOT;
use crate::internals;
use log::{error, info};
use samp::amx::AmxIdent;
use samp::prelude::*;
use std::collections::{HashMap, LinkedList};

pub struct TgConnector {
    pub plugin_version: i32,
    pub amx_list: Vec<AmxIdent>,
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

impl SampPlugin for TgConnector {
    fn on_load(&mut self) {
        info!(
            "
   ###############################################################
   #                      TGConnector                            #
   #                        v0.2.0 Loaded!!                      #
   #   Found any bugs? Report it here:                           #
   #       https://github.com/Sreyas-Sreelal/tgconnector/issues  #
   #                                                             #
   ###############################################################
			"
        );
    }

    fn on_unload(self: Box<TgConnector>) {
        info!("**TGConnector v0.2.0 Unloaded!");
    }

    fn on_amx_load(&mut self, amx: &Amx) {
        self.amx_list.push(amx.ident());

        let get_version = amx.find_pubvar::<i32>("_tgconnector_version");

        match get_version {
            Ok(version) => {
                if *version != self.plugin_version {
                    info!("Warning plugin and include version doesnot match : Include {:?} Plugin {:?}",*version,self.plugin_version);
                }
            }
            Err(err) => error!("Failed to retrive include version Reason:{:?}", err),
        }
    }

    fn on_amx_unload(&mut self, amx: &Amx) {
        let raw = amx.ident();
        let index = self.amx_list.iter().position(|x| *x == raw).unwrap();
        self.amx_list.remove(index);
    }

    fn process_tick(&mut self) {
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
