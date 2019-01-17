#define RUN_TESTS

#include <a_samp>
#include <YSI\y_testing>

#include "../include/tgconnector.inc"

Test:TestInvalidToken() {
	new TGBot:bot = TGConnect("");
	ASSERT(bot == TGBot:-1);
}

Test:TestValidToken() {
	new TGBot:bot = TGConnectFromEnv("SAMP_TG_BOT");
	printf("id is %d",_:bot);
	ASSERT(bot != TGBot:-1);
}

main(){

}

public OnTGMessage(TGBot:bot,TGUser:fromid,TGMessage:messageid) {
	new 
		TGChatid:chatid[34],
		message[128],
		chattype[15],
		username[24],
		chatname[56];

	TGGetChatId(chatid);
	TGGetMessage(message);
	TGGetUserName(username);
	TGGetChatName(chatname);
	TGGetChatType(chattype);
	
	printf("chattid: %s chatname:%s chattype:%s",_:chatid,chatname,chattype);
	printf("username:%s message:%s messageid:%d\n",username,message,_:messageid);
	
	if(!strcmp("562896556",_:chatid)){
		TGSendMessage(bot,chatid,message,messageid);
	}
	
}