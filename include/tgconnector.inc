#if defined _inc_tgconnector
	#undef _inc_tgconnector
#endif

#if defined _tgconnector_included
	#endinput
#endif
#define _tgconnector_included

native TGBot:TGConnect(const token[]);
native TGBot:TGConnectFromEnv(const variable[]);
native TGSendMessage(TGBot:bot,const TGChatid:chatid[],const text[]);
native TGGetUserName(str[],size=sizeof(str));
native TGGetChatName(str[],size=sizeof(str));
native TGGetChatType(str[],size=sizeof(str));
native TGGetChatId(TGChatid:str[],size=sizeof(str));
native TGGetMessage(str[],size=sizeof(str));

forward OnTGMessage(TGBot:bot,TGUser:fromid,TGMessage:messageid);