#if defined _inc_tgconnector
	#undef _inc_tgconnector
#endif

#if defined _tgconnector_included
	#endinput
#endif
#define _tgconnector_included

#define INVALID_MESSAGE_ID (TGMessage:-1)
#define INVALID_BOT_ID (TGBot:-1)

enum TGParseMode {
	HTML,
	MARKDOWN
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
native TGBot:TGConnect(const token[]);
native TGBot:TGConnectFromEnv(const variable[]);
native TGSendMessage(TGBot:bot,const TGChatid:chatid[],const text[],TGMessage:reply_id=INVALID_MESSAGE_ID,TGParseMode:parse_mode=TGParseMode:-1,callback[]="");
native TGDeleteMessage(TGBot:bot,TGChatid:chatid[],TGMessage:messageid);
native TGEditMessage(TGBot:bot,TGChatid:chatid[],TGMessage:messageid,text[],TGParseMode:parse_mode=TGParseMode:-1);

//user
native TGUserStatus:TGGetUserChatStatus(TGBot:bot,TGUser:userid,TGChatid:chatid[]);
native TGGetUserNameFromId(TGBot:bot,TGUser:userid,TGChatid:chatid[],username[],size=sizeof(username));
native TGGetDisplayNameFromId(TGBot:bot,TGUser:userid,TGChatid:chatid[],displayname[],size=sizeof(displayname));

//chat 
native TGGetChatMembersCount(TGBot:bot,TGChatid:chatid[]);
native TGGetChatTitle(TGBot:bot,TGChatid:chatid[],title[],size=sizeof(title));
native TGGetChatDescription(TGBot:bot,TGChatid:chatid[],description[],size=sizeof(description));

//cache functions
native TGCacheGetUserFirstName(str[],size=sizeof(str));
native TGCacheGetUserLastName(str[],size=sizeof(str));
native TGCacheGetUserName(str[],size=sizeof(str));
native TGCacheGetChatName(str[],size=sizeof(str));
native TGCacheGetChatType(str[],size=sizeof(str));
native TGCacheGetChatId(TGChatid:str[],size=sizeof(str));
native TGCacheGetMessage(str[],size=sizeof(str));

//callbacks
forward OnTGMessage(TGBot:bot,TGUser:fromid,TGMessage:messageid);
forward OnTgUserJoined(TGBot:bot,TGUser:userid);
forward OnTgUserLeft(TGBot:bot,TGUser:userid);
