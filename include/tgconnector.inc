#if defined _inc_tgconnector
	#undef _inc_tgconnector
#endif

#if defined _tgconnector_included
	#endinput
#endif
#define _tgconnector_included

#define INVALID_MESSAGE_ID (TGMessage:-1)
#define INVALID_BOT_ID (TGBot:-1)

native TGBot:TGConnect(const token[]);
native TGBot:TGConnectFromEnv(const variable[]);
native TGSendMessage(TGBot:bot,const TGChatid:chatid[],const text[],TGMessage:reply_id=INVALID_MESSAGE_ID);
native TGGetUserName(str[],size=sizeof(str));
native TGGetChatName(str[],size=sizeof(str));
native TGGetChatType(str[],size=sizeof(str));
native TGGetChatId(TGChatid:str[],size=sizeof(str));
native TGGetMessage(str[],size=sizeof(str));

forward OnTGMessage(TGBot:bot,TGUser:fromid,TGMessage:messageid);