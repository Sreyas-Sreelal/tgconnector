#define RUN_TESTS

#include <a_samp>
#include <YSI_Core\y_testing>

#include "../include/tgconnector.inc"

new TGBot:g_bot;

main() {
	g_bot = TG_ConnectFromEnv("SAMP_TG_BOT");
}

Test:TestInvalidToken() {
	new TGBot:invalidbot = TG_Connect("");
	ASSERT(invalidbot == INVALID_BOT_ID);
}

Test:TestValidToken() {
	printf("id is %d",_:g_bot);
	TG_SendMessage(g_bot,TGChatId:"@testtgconnector","`markdown text` ***bold*** _italic_ 123",.parse_mode=MARKDOWN,.callback="SendingMessage");
	TG_SendMessage(g_bot,TGChatId:"@testtgconnector","__underline__",.parse_mode=MARKDOWN2);
	
	ASSERT(g_bot != INVALID_BOT_ID);
}
Test:TG_GetBotUserID() {
	new
		TGUser:userid[64],
		name[34],
		username[32];
	TG_GetBotUserID(g_bot,userid);
	TG_GetDisplayNameFromID(g_bot,userid,TGChatId:"562896556",name);
	new bool:name_check = !strcmp("samp",name);
	ASSERT(name_check);

	TG_GetUserNameFromID(g_bot,userid,TGChatId:"562896556",username);
	new bool:username_check = !strcmp("samptg_bot",username);
	ASSERT(username_check);

	new TGUserStatus:status = TG_GetUserChatStatus(g_bot,userid,TGChatId:"562896556");
	ASSERT(status == TG_MEMBER);

}

Test:TG_GetUserChatStatus() {
	new TGUserStatus:status = TG_GetUserChatStatus(g_bot,TGUser:"562896556",TGChatId:"-1001961091419");
	ASSERT(status == TG_CREATOR);
}

Test:TG_GetChatMembersCount() {
	new count = TG_GetChatMembersCount(g_bot,TGChatId:"-1001961091419");
	printf("count %d",count);
	ASSERT(count == 2);
}

Test:TG_GetUserNameFromId() {
	new username[32];
	TG_GetUserNameFromID(g_bot,TGUser:"562896556",TGChatId:"562896556",username);
	new check = !strcmp("SyS54",username) && username[0] != '\0';
	ASSERT(check == 1);
}

Test:TG_GetDisplayNameFromId() {
	new displayname[32];
	TG_GetDisplayNameFromID(g_bot,TGUser:"562896556",TGChatId:"562896556",displayname);
	new check = !strcmp("Crow's Eye",displayname) && displayname[0] != '\0';
	ASSERT(check == 1);
}

Test:TG_GetChatTitle() {
	new title[132];
	TG_GetChatTitle(g_bot,TGChatId:"-1001961091419",title);
	printf("title : %s",title);
	new check = !strcmp("bot_developement",title) && title[0] != '\0';
	ASSERT(check == 1);
}

Test:TG_GetChatDescription() {
	new description[132];
	TG_GetChatDescription(g_bot,TGChatId:"-1001961091419",description);
	printf("description : %s",description);
	new check = !strcmp("testing bots",description) && description[0] != '\0';
	ASSERT(check == 1);
}

public OnTGMessage(TGBot:bot,TGUser:fromid[],TGMessage:messageid) {
	new
		TGChatId:chatid[15],
		message[128],
		chattype[15],
		username[24],
		chatname[56],
		firstname[34],
		lastname[34];

	TG_CacheGetChatID(chatid);
	TG_CacheGetMessage(message);
	TG_CacheGetUserName(username);
	TG_CacheGetChatName(chatname);
	TG_CacheGetChatType(chattype);
	TG_CacheGetUserLastName(lastname);
	TG_CacheGetUserFirstName(firstname);

	printf("chattid: %s chatname:%s chattype:%s",_:chatid,chatname,chattype);
	printf("userid:%d username:%s firstname:%s lastname:%s message:%s messageid:%d\n",_:fromid,username,firstname,lastname,message,_:messageid);

	TG_DeleteMessage(bot,chatid,messageid);
	TG_SendMessage(bot,chatid,message,.callback="SendingMessage");
	return 1;
}

forward SendingMessage(TGBot:bot,TGMessage:messageid);
public SendingMessage(TGBot:bot,TGMessage:messageid) {
	new TGChatId:chatid[15];
	TG_CacheGetChatID(chatid);
	TG_EditMessage(bot,chatid,messageid,"***edited message***",.parse_mode=MARKDOWN);
	return 1;
}
public OnTGChannelPost(TGBot:bot,TGMessage:postid) {
	new
		post[200],
		chatname[56],
		TGChatId:chatid[15];

	TG_CacheGetMessage(post);
	TG_CacheGetChatName(chatname);
	TG_CacheGetChatID(chatid);

	printf("[%s](%s):%s(%d)",chatname,_:chatid,post,_:postid);
}
public OnTGUserJoined(TGBot:bot,TGUser:userid[]) {
	new
		TGChatId:chatid[15],
		username[24],
		chatname[129];

	TG_CacheGetUserName(username);
	TG_CacheGetChatID(chatid);
	TG_CacheGetChatName(chatname);

	printf("User %s(%s) joined %s(%s)",username,_:userid,chatname,_:chatid);
	return 1;
}

public OnTGUserLeft(TGBot:bot,TGUser:userid[]) {
	new
		TGChatId:chatid[15],
		username[24],
		chatname[129];

	TG_CacheGetUserName(username);
	TG_CacheGetChatID(chatid);
	TG_CacheGetChatName(chatname);

	printf("User %s(%s) left %s(%s)",username,_:userid,chatname,_:chatid);
	return 1;
}