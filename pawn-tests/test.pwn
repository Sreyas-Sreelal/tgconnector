#define RUN_TESTS

#include <a_samp>
#include <YSI\y_testing>

#include "../include/tgconnector.inc"

new TGBot:g_bot;

main() {
	g_bot = TGConnectFromEnv("SAMP_TG_BOT");
}

Test:TestInvalidToken() {
	new TGBot:invalidbot = TGConnect("");
	ASSERT(invalidbot == INVALID_BOT_ID);
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
		chatname[56],
		firstname[34],
		lastname[34];

	TGCacheGetChatId(chatid);
	TGCacheGetMessage(message);
	TGCacheGetUserName(username);
	TGCacheGetChatName(chatname);
	TGCacheGetChatType(chattype);
	TGCacheGetUserLastName(lastname);
	TGCacheGetUserFirstName(firstname);

	printf("chattid: %s chatname:%s chattype:%s",_:chatid,chatname,chattype);
	printf("username:%s firstname:%s lastname:%s message:%s messageid:%d\n",username,firstname,lastname,message,_:messageid);
	
	//TGDeleteMessage(bot,chatid,messageid);
	TGSendMessage(bot,chatid,message,messageid,.callback="SendingMessage");
	return 1;
}

forward SendingMessage(TGBot:bot,TGMessage:messageid);
public SendingMessage(TGBot:bot,TGMessage:messageid) {
	new TGChatid:chatid[52];
	TGCacheGetChatId(chatid);
	TGEditMessage(bot,chatid,messageid,"***edited message***",.parse_mode=MARKDOWN);
	return 1;
}

public OnTgUserJoined(TGBot:bot,TGUser:userid) {
	new 
		TGChatid:chatid[23],
		username[24],
		chatname[129];
	
	TGCacheGetUserName(username);
	TGCacheGetChatId(chatid);
	TGCacheGetChatName(chatname);

	printf("User %s(%d) joined %s(%s)",username,_:userid,chatname,_:chatid);
	return 1;
}

public OnTgUserLeft(TGBot:bot,TGUser:userid) {
	new 
		TGChatid:chatid[23],
		username[24],
		chatname[129];
	
	TGCacheGetUserName(username);
	TGCacheGetChatId(chatid);
	TGCacheGetChatName(chatname);

	printf("User %s(%d) left %s(%s)",username,_:userid,chatname,_:chatid);
	return 1;
}