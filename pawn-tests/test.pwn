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

public OnTGMessage(TGBot:bot,TGUser:fromid) {
	new 
		TGChatid:chatid[34],
		message[25],
		chattype[55],
		username[24],
		chatname[56];

	TGGetChatId(chatid);
	TGGetUserName(username);
	TGGetChatName(chatname);
	TGGetChatType(chattype);
	TGGetMessage(message);
	
	printf("chattid: %s chatname:%s chattype:%s username:%s message:%s",_:chatid,chatname,chattype,username,message);
	
	if(!strcmp("562896556",_:chatid)){
		TGSendMessage(bot,chatid,message);
	}
	
}