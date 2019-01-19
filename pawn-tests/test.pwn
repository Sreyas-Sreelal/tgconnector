#define RUN_TESTS

#include <a_samp>
#include <YSI\y_testing>

#include "../include/tgconnector.inc"

new TGBot:g_bot;

Test:TestInvalidToken() {
	new TGBot:invalidbot = TGConnect("");
	ASSERT(invalidbot == INVALID_BOT_ID);
}
main() {
	g_bot = TGConnectFromEnv("SAMP_TG_BOT");
}

Test:TestValidToken() {
	printf("id is %d",_:g_bot);
	TGSendMessage(g_bot,TGChatid:"562896556","`markdown text` ***bold*** _italic_ 123",.parse_mode=MARKDOWN,.callback="SendingMessage");
	ASSERT(g_bot != INVALID_BOT_ID);
}

Test:TGGetUserGroupStatus() {
	new TGUserStatus:status = TGGetUserGroupStatus(g_bot,TGUser:562896556,TGChatid:"-1001445898764");
	printf("status %d",_:status);
	ASSERT(status == TG_CREATOR);
}


public OnTGMessage(TGBot:bot,TGUser:fromid,TGMessage:messageid) {
	new 
		TGChatid:chatid[34],
		message[128],
		chattype[15],
		username[24],
		chatname[56];

	TGGetChatId(chatid);
	TGGetMessage(message);
	TGGetUserName(username);
	TGGetChatName(chatname);
	TGGetChatType(chattype);
	
	printf("chattid: %s chatname:%s chattype:%s",_:chatid,chatname,chattype);
	printf("username:%s message:%s messageid:%d\n",username,message,_:messageid);
	
	TGDeleteMessage(bot,chatid,messageid);
	TGSendMessage(bot,chatid,message,messageid,.callback="SendingMessage");
}

forward SendingMessage(TGBot:bot,TGMessage:messageid);
public SendingMessage(TGBot:bot,TGMessage:messageid) {
	new TGChatid:chatid[52];
	TGGetChatId(chatid);
	TGEditMessage(bot,chatid,messageid,"***edited message***",.parse_mode=MARKDOWN);
}

public OnTgUserJoined(TGBot:bot,TGUser:userid) {
	new 
		TGChatid:chatid[23],
		username[24],
		chatname[129];
	
	TGGetUserName(username);
	TGGetChatId(chatid);
	TGGetChatName(chatname);

	printf("User %s(%d) joined %s(%s)",username,_:userid,chatname,_:chatid);
}

public OnTgUserLeft(TGBot:bot,TGUser:userid) {
	new 
		TGChatid:chatid[23],
		username[24],
		chatname[129];
	
	TGGetUserName(username);
	TGGetChatId(chatid);
	TGGetChatName(chatname);

	printf("User %s(%d) left %s(%s)",username,_:userid,chatname,_:chatid);
}