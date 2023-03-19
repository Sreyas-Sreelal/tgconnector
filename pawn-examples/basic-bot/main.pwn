#include<a_samp>
#include<tgconnector>
#include<zcmd>

#define CHAT_ID (TGChatId:"-1001445898764")

new TGBot:g_bot;

main() {
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
	print(server_msg);
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

	format(server_msg,128,"User %s(%s) joined %s(%s)",username,_:userid,chatname,_:chatid);
	print(server_msg);
	SendClientMessageToAll(-1,server_msg);
	return 1;
}

public OnTGUserLeft(TGBot:bot,TGUser:userid) {
	new
		TGChatId:chatid[12],
		username[24],
		chatname[56],
		server_msg[128];

	TG_CacheGetUserName(username);
	TG_CacheGetChatId(chatid);
	TG_CacheGetChatName(chatname);

	format(server_msg,128,"User %s(%s) left %s(%s)",username,_:userid,chatname,_:chatid);
	print(server_msg);
	SendClientMessageToAll(-1,server_msg);
	return 1;
}

CMD:sendtgmessage(playerid,params[]) {
	TG_SendMessage(g_bot,CHAT_ID,params);
	return 1;
}