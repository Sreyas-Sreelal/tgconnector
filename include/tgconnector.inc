#if defined _inc_tgconnector
	#undef _inc_tgconnector
#endif

#if defined _tgconnector_included
	#endinput
#endif
#define _tgconnector_included

#define TGCONNECTOR_VERSION 31
#define INVALID_MESSAGE_ID (TGMessage:-1)
#define INVALID_BOT_ID (TGBot:-1)

public _tgconnector_version = TGCONNECTOR_VERSION;
#pragma unused _tgconnector_version

enum TGParseMode {
	HTML,
	MARKDOWN,
	MARKDOWN2
};

enum TGUserStatus {
	TG_CREATOR = 1,
	TG_ADMINISTRATOR,
	TG_MEMBER,
	TG_RESTRICTED,
	TG_LEFT,
	TG_KICKED,
};

//basic
native TGBot:TGConnect(const token[],const proxy[]="",thread_limit=3);
native TGBot:TGConnectFromEnv(const variable[],const proxy[]="",thread_limit=3);
native TGSendMessage(TGBot:bot,const TGChatId:chatid[],const text[],TGMessage:reply_id=INVALID_MESSAGE_ID,TGParseMode:parse_mode=TGParseMode:-1,callback[]="");
native TGDeleteMessage(TGBot:bot,TGChatId:chatid[],TGMessage:messageid);
native TGEditMessage(TGBot:bot,TGChatId:chatid[],TGMessage:messageid,text[],TGParseMode:parse_mode=TGParseMode:-1);

//bot
native TGUser:TGGetBotUserId(TGBot:bot);

//user
native TGUserStatus:TGGetUserChatStatus(TGBot:bot,TGUser:userid,TGChatId:chatid[]);
native TGGetUserNameFromId(TGBot:bot,TGUser:userid,TGChatId:chatid[],username[],size=sizeof(username));
native TGGetDisplayNameFromId(TGBot:bot,TGUser:userid,TGChatId:chatid[],displayname[],size=sizeof(displayname));

//chat
native TGGetChatMembersCount(TGBot:bot,TGChatId:chatid[]);
native TGGetChatTitle(TGBot:bot,TGChatId:chatid[],title[],size=sizeof(title));
native TGGetChatDescription(TGBot:bot,TGChatId:chatid[],description[],size=sizeof(description));

//cache functions
native TGCacheGetUserFirstName(str[],size=sizeof(str));
native TGCacheGetUserLastName(str[],size=sizeof(str));
native TGCacheGetUserName(str[],size=sizeof(str));
native TGCacheGetChatName(str[],size=sizeof(str));
native TGCacheGetChatType(str[],size=sizeof(str));
native TGCacheGetChatId(TGChatId:str[],size=sizeof(str));
native TGCacheGetMessage(str[],size=sizeof(str));

//callbacks
forward OnTGMessage(TGBot:bot,TGUser:fromid,TGMessage:messageid);
forward OnTGChannelPost(TGBot:bot,TGMessage:postid);
forward OnTGUserJoined(TGBot:bot,TGUser:userid);
forward OnTGUserLeft(TGBot:bot,TGUser:userid);
