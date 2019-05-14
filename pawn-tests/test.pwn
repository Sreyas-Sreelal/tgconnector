#define RUN_TESTS

#include <a_samp>
#include <YSI_Core\y_testing>

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
	TGSendMessage(g_bot,TGChatId:"@testsamp","`markdown text` ***bold*** _italic_ 123",.parse_mode=MARKDOWN,.callback="SendingMessage");
	ASSERT(g_bot != INVALID_BOT_ID);
}
Test:TGGetBotUserId() {
	new
		TGUser:userid = TGGetBotUserId(g_bot),
		name[34],
		username[32];

	TGGetDisplayNameFromId(g_bot,userid,TGChatId:"562896556",name);
	new bool:name_check = !strcmp("samp",name);
	ASSERT(name_check);

	TGGetUserNameFromId(g_bot,userid,TGChatId:"562896556",username);
	new bool:username_check = !strcmp("samptg_bot",username);
	ASSERT(username_check);

	new TGUserStatus:status = TGGetUserChatStatus(g_bot,userid,TGChatId:"562896556");
	ASSERT(status == TG_MEMBER);

}

Test:TGGetUserChatStatus() {
	new TGUserStatus:status = TGGetUserChatStatus(g_bot,TGUser:562896556,TGChatId:"-1001445898764");
	ASSERT(status == TG_CREATOR);
}

Test:TGGetChatMembersCount() {
	new count = TGGetChatMembersCount(g_bot,TGChatId:"-1001445898764");
	printf("count %d",count);
	ASSERT(count == 8);
}

Test:TGGetUserNameFromId() {
	new username[32];
	TGGetUserNameFromId(g_bot,TGUser:562896556,TGChatId:"562896556",username);
	new check = !strcmp("SyS45",username) && username[0] != '\0';
	ASSERT(check == 1);
}

Test:TGGetDisplayNameFromId() {
	new displayname[32];
	TGGetDisplayNameFromId(g_bot,TGUser:562896556,TGChatId:"562896556",displayname);
	new check = !strcmp("__SyS",displayname) && displayname[0] != '\0';
	ASSERT(check == 1);
}

Test:TGGetChatTitle() {
	new title[132];
	TGGetChatTitle(g_bot,TGChatId:"-1001445898764",title);
	printf("title : %s",title);
	new check = !strcmp("bot_developement",title) && title[0] != '\0';
	ASSERT(check == 1);
}

Test:TGGetChatDescription() {
	new description[132];
	TGGetChatDescription(g_bot,TGChatId:"-1001445898764",description);
	printf("description : %s",description);
	new check = !strcmp("testing bots",description) && description[0] != '\0';
	ASSERT(check == 1);
}

public OnTGMessage(TGBot:bot,TGUser:fromid,TGMessage:messageid) {
	new
		TGChatId:chatid[15],
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
	printf("userid:%d username:%s firstname:%s lastname:%s message:%s messageid:%d\n",_:fromid,username,firstname,lastname,message,_:messageid);

	TGDeleteMessage(bot,chatid,messageid);
	TGSendMessage(bot,chatid,message,.callback="SendingMessage");
	return 1;
}

forward SendingMessage(TGBot:bot,TGMessage:messageid);
public SendingMessage(TGBot:bot,TGMessage:messageid) {
	new TGChatId:chatid[15];
	TGCacheGetChatId(chatid);
	TGEditMessage(bot,chatid,messageid,"***edited message***",.parse_mode=MARKDOWN);
	return 1;
}
public OnTGChannelPost(TGBot:bot,TGMessage:postid) {
	new
		post[200],
		chatname[56],
		TGChatId:chatid[15];

	TGCacheGetMessage(post);
	TGCacheGetChatName(chatname);
	TGCacheGetChatId(chatid);

	printf("[%s](%s):%s(%d)",chatname,_:chatid,post,_:postid);
}
public OnTGUserJoined(TGBot:bot,TGUser:userid) {
	new
		TGChatId:chatid[15],
		username[24],
		chatname[129];

	TGCacheGetUserName(username);
	TGCacheGetChatId(chatid);
	TGCacheGetChatName(chatname);

	printf("User %s(%d) joined %s(%s)",username,_:userid,chatname,_:chatid);
	return 1;
}

public OnTGUserLeft(TGBot:bot,TGUser:userid) {
	new
		TGChatId:chatid[15],
		username[24],
		chatname[129];

	TGCacheGetUserName(username);
	TGCacheGetChatId(chatid);
	TGCacheGetChatName(chatname);

	printf("User %s(%d) left %s(%s)",username,_:userid,chatname,_:chatid);
	return 1;
}