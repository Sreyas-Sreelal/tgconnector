#define RUN_TESTS

#include <a_samp>
#include <YSI\y_testing>

#include "../include/tgconnector.inc"

Test:TestInvalidToken() {
    new TGBot:bot = TGConnect("");
    ASSERT(bot == TGBot:-1);
}

Test:TestValidToken() {
    new TGBot:bot = TGConnectFromEnv("CIRILLA_TOKEN");
    printf("id is %d",_:bot);
    ASSERT(bot != TGBot:-1);
}

main(){

}
public OnTGMessage(
	TGBot:bot,
	TGUser:fromid,
	const TGChatid:chatid[],
	const username[],
	const chatname[],
	const chattype[],
	const message[]
) {
    printf("[%s] [%s] : %s",chatname,username,message);
}