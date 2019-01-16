#if defined _inc_tgconnector
	#undef _inc_tgconnector
#endif

#if defined _tgconnector_included
	#endinput
#endif
#define _tgconnector_included

native TGBot:TGConnect(const token[]);

native TGBot:TGConnectFromEnv(const variable[]);

forward OnTGMessage(TGBot:bot,TGUser:fromid,const TGChatid:chatid[],const string[]);

