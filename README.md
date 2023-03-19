# TgConnector
[![Build Status](https://travis-ci.org/Sreyas-Sreelal/tgconnector.svg?branch=master)](https://travis-ci.org/Sreyas-Sreelal/tgconnector) [![Build status](https://ci.appveyor.com/api/projects/status/snip8i9cd6xh2x1u?svg=true)](https://ci.appveyor.com/project/Sreyas-Sreelal/tgconnector)
[![sampctl-supported](https://shields.southcla.ws/badge/sampctl-TGConnector-2f2f2f.svg)](https://github.com/Sreyas-Sreelal/tgconnector)
[![GitHub issues](https://img.shields.io/github/issues/Sreyas-Sreelal/tgconnector.svg)](https://github.com/Sreyas-Sreelal/tgconnector/issues) [![GitHub pull requests](https://img.shields.io/github/issues-pr-raw/sreyas-sreelal/tgconnector.svg)](https://github.com/Sreyas-Sreelal/tgconnector/pulls) [![GitHub pull license](https://img.shields.io/github/license/sreyas-sreelal/tgconnector.svg)](LICENSE)

A telegram connector plugin that helps to interact with telgram bots through SA-MP.
## Installing

If you are a sampctl user

`sampctl p install Sreyas-Sreelal/tgconnector`

#### OR
* Download suitable binary files from releases for your operating system
* Add it your `plugins` folder
* Add `tgconnector` to server.cfg or  `tgconnector.so` (for linux)
* Add [tgconnector.inc](include/tgconnector.inc) in includes folder

## Building
* Clone the repo

	`git clone https://github.com/Sreyas-Sreelal/tgconnector.git`

* Use makefile to compile and test
	* Setup testing environment

		`make setup`
	* To build release version

		`make release`
	* Run tests

		`make run`

## API

Checkout the [Wiki](https://github.com/Sreyas-Sreelal/tgconnector/wiki)

## Example
**A basic bot**
```Pawn
#include<a_samp>
#include<tgconnector>
#include<zcmd>

#define CHAT_ID (TGChatId:"YOUR_CHAT_ID_HERE")

new TGBot:g_bot;

main() {
	//Store bot token in SAMP_TG_BOT environment variable and connect from it
	g_bot = TG_ConnectFromEnv("SAMP_TG_BOT");
	if(g_bot != INVALID_BOT_ID) {
		printf("bot connected successfully!");
	} else {
		printf("Error: bot couldn't connect");
	}
}

public OnTGMessage(TGBot:bot,TGUser:fromid[],TGMessage:messageid) {

	if(g_bot != bot){
		return 1;
	}

	new
		message[50],
		username[24],
		chatname[56],
		server_msg[128];

	TG_CacheGetMessage(message);
	TG_CacheGetUserName(username);
	TG_CacheGetChatName(chatname);

	format(server_msg,128,"[%s] %s(%s): %s",chatname,username,_:fromid,message);
	SendClientMessageToAll(-1,server_msg);

	return 1;
}


public OnTGUserJoined(TGBot:bot,TGUser:userid[]) {
	new
		TGChatId:chatid[12],
		username[24],
		chatname[56],
		server_msg[128];

	TG_CacheGetUserName(username);
	TG_CacheGetChatId(chatid);
	TG_CacheGetChatName(chatname);

	format(server_msg,128,"User %s(%d) joined %s(%s)",username,_:userid,chatname,_:chatid);
	SendClientMessageToAll(-1,server_msg);
	return 1;
}

public OnTGUserLeft(TGBot:bot,TGUser:userid[]) {
	new
		TGChatId:chatid[12],
		username[24],
		chatname[56],
		server_msg[128];

	TG_CacheGetUserName(username);
	TG_CacheGetChatID(chatid);
	TG_CacheGetChatName(chatname);

	format(server_msg,128,"User %s(%s) left %s(%s)",username,_:userid,chatname,_:chatid);
	SendClientMessageToAll(-1,server_msg);
	return 1;
}

CMD:sendtgmessage(playerid,params[]) {
	TG_SendMessage(g_bot,CHAT_ID,params);
	return 1;
}
```