#if defined _inc_tgconnector
	#undef _inc_tgconnector
#endif

#if defined _tgconnector_included
	#endinput
#endif
#define _tgconnector_included

#define TGCONNECTOR_VERSION 110
#define INVALID_MESSAGE_ID (TGMessage:-1)
#define INVALID_BOT_ID (TGBot:-1)
#define INVALID_TOPIC_ID (TGTopic:-1)

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

// old aliases
native TGBot:TGConnect(const token[],const proxy[]="",thread_limit=3)=TG_Connect;
native TGBot:TGConnectFromEnv(const variable[],const proxy[]="",thread_limit=3)=TG_ConnectFromEnv;
native TGSendMessage(TGBot:bot,const TGChatId:chatid[],const text[],TGMessage:reply_id=INVALID_MESSAGE_ID,TGParseMode:parse_mode=TGParseMode:-1,bool:disable_web_page_preview=false,const callback[]="") = TG_SendMessage;
native TGDeleteMessage(TGBot:bot,TGChatId:chatid[],TGMessage:messageid)=TG_DeleteMessage;
native TGEditMessage(TGBot:bot,const TGChatId:chatid[],const TGMessage:messageid,const text[],TGParseMode:parse_mode=TGParseMode:-1)=TG_EditMessage;
native TGGetChatMembersCount(TGBot:bot,const TGChatId:chatid[])=TG_GetChatMembersCount;
native TGGetChatTitle(TGBot:bot,const TGChatId:chatid[],title[],size=sizeof(title)) = TG_GetChatTitle;
native TGGetChatDescription(TGBot:bot,const TGChatId:chatid[],description[],size=sizeof(description)) = TG_GetChatDescription;
native TGCacheGetUserFirstName(str[],size=sizeof(str)) = TG_CacheGetUserFirstName;
native TGCacheGetUserLastName(str[],size=sizeof(str)) = TG_CacheGetUserLastName;
native TGCacheGetUserName(str[],size=sizeof(str)) = TG_CacheGetUserName;
native TGCacheGetChatName(str[],size=sizeof(str)) = TG_CacheGetChatName;
native TGCacheGetChatType(str[],size=sizeof(str)) = TG_CacheGetChatType;
native TGCacheGetChatId(TGChatId:str[],size=sizeof(str)) = TG_CacheGetChatID;
native TGCacheGetMessage(str[],size=sizeof(str)) = TG_CacheGetMessage;

//basic
native TGBot:TG_Connect(const token[],const proxy[]="",thread_limit=3);
native TGBot:TG_ConnectFromEnv(const variable[],const proxy[]="",thread_limit=3);
native TG_SendMessage(TGBot:bot,const TGChatId:chatid[],const text[],TGMessage:reply_id=INVALID_MESSAGE_ID,TGParseMode:parse_mode=TGParseMode:-1,bool:disable_web_page_preview=false,const callback[]="");
native TG_DeleteMessage(TGBot:bot,TGChatId:chatid[],TGMessage:messageid);
native TG_EditMessage(TGBot:bot,const TGChatId:chatid[],const TGMessage:messageid,const text[],TGParseMode:parse_mode=TGParseMode:-1);
native TG_BanChatMember(TGBot:bot,const TGChatId:chatid[],const TGUser:userid[],until_date=-1,bool:revoke_messages=true);
native TG_UnbanChatMember(TGBot:bot,const TGChatId:chatid[],const TGUser:userid[],only_if_banned=false);

//bot
native TGUser:TG_GetBotUserID(TGBot:bot,TGUser:userid[],size = sizeof(userid));

//user
native TGUserStatus:TG_GetUserChatStatus(TGBot:bot,const TGUser:userid[],const TGChatId:chatid[]);
native TG_GetUserNameFromID(TGBot:bot,const TGUser:userid[],const TGChatId:chatid[],username[],size=sizeof(username));
native TG_GetDisplayNameFromID(TGBot:bot,const TGUser:userid[],const TGChatId:chatid[],displayname[],size=sizeof(displayname));

//chat
native TG_GetChatMembersCount(TGBot:bot,const TGChatId:chatid[]);
native TG_GetChatTitle(TGBot:bot,const TGChatId:chatid[],title[],size=sizeof(title));
native TG_GetChatDescription(TGBot:bot,const TGChatId:chatid[],description[],size=sizeof(description));

//cache functions
native TG_CacheGetUserFirstName(str[],size=sizeof(str));
native TG_CacheGetUserLastName(str[],size=sizeof(str));
native TG_CacheGetUserName(str[],size=sizeof(str));
native TG_CacheGetChatName(str[],size=sizeof(str));
native TG_CacheGetChatType(str[],size=sizeof(str));
native TG_CacheGetChatID(TGChatId:str[],size=sizeof(str));
native TG_CacheGetMessage(str[],size=sizeof(str));

//callbacks
forward OnTGMessage(TGBot:bot,TGUser:fromid[],TGMessage:messageid,TGTopic:messagethreadid);
forward OnTGChannelPost(TGBot:bot,TGMessage:postid);
forward OnTGUserJoined(TGBot:bot,TGUser:userid[]);
forward OnTGUserLeft(TGBot:bot,TGUser:userid[]);