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

native TGBot:TGConnect(const token[]);
native TGBot:TGConnectFromEnv(const variable[]);
native TGSendMessage(TGBot:bot,const TGChatid:chatid[],const text[],TGMessage:reply_id=INVALID_MESSAGE_ID,TGParseMode:parse_mode=TGParseMode:-1,callback[]="");
native TGGetUserFirstName(str[],size=sizeof(str));
native TGGetUserLastName(str[],size=sizeof(str));
native TGGetUserName(str[],size=sizeof(str));
native TGGetChatName(str[],size=sizeof(str));
native TGGetChatType(str[],size=sizeof(str));
native TGGetChatId(TGChatid:str[],size=sizeof(str));
native TGGetMessage(str[],size=sizeof(str));
native TGDeleteMessage(TGBot:bot,TGChatid:chatid[],TGMessage:messageid);
native TGEditMessage(TGBot:bot,TGChatid:chatid[],TGMessage:messageid,text[],TGParseMode:parse_mode=TGParseMode:-1);
native TGUserStatus:TGGetUserGroupStatus(TGBot:bot,TGUser:userid,TGChatid:chatid[]);

forward OnTGMessage(TGBot:bot,TGUser:fromid,TGMessage:messageid);
forward OnTgUserJoined(TGBot:bot,TGUser:userid);
forward OnTgUserLeft(TGBot:bot,TGUser:userid);
