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

_Coming Soon_

## Example
**A basic bot**
```Pawn
#include<a_samp>
#include<tgconnector>
#include<zcmd>

#define CHAT_ID (TGChatId:"-1001445898764")

new TGBot:g_bot;

main() {
	g_bot = TGConnectFromEnv("SAMP_TG_BOT");
	if(g_bot != INVALID_BOT_ID) {
		printf("bot connected successfully!");
	} else {
		printf("Error: bot couldn't connect");
	}
}

public OnTGMessage(TGBot:bot,TGUser:fromid,TGMessage:messageid) {
	
	if(g_bot != bot){
		return 1;
	}

	new 
		message[50],
		username[24],
		chatname[56],
		server_msg[128];

	TGCacheGetMessage(message);
	TGCacheGetUserName(username);
	TGCacheGetChatName(chatname);
	
	format(server_msg,128,"[%s] %s(%d): %s",chatname,username,_:fromid,message);
	SendClientMessageToAll(-1,server_msg);
	
	return 1;
}


public OnTgUserJoined(TGBot:bot,TGUser:userid) {
	new 
		TGChatId:chatid[12],
		username[24],
		chatname[56],
		server_msg[128];
	
	TGCacheGetUserName(username);
	TGCacheGetChatId(chatid);
	TGCacheGetChatName(chatname);

	format(server_msg,128,"User %s(%d) joined %s(%s)",username,_:userid,chatname,_:chatid);
	SendClientMessageToAll(-1,server_msg);
	return 1;
}

public OnTgUserLeft(TGBot:bot,TGUser:userid) {
	new 
		TGChatId:chatid[12],
		username[24],
		chatname[56],
		server_msg[128];
	
	TGCacheGetUserName(username);
	TGCacheGetChatId(chatid);
	TGCacheGetChatName(chatname);

	format(server_msg,128,"User %s(%d) left %s(%s)",username,_:userid,chatname,_:chatid);
	SendClientMessageToAll(-1,server_msg);
	return 1;
}

CMD:sendtgmessage(playerid,params[]) {
	TGSendMessage(g_bot,CHAT_ID,params);
	return 1;
}
```