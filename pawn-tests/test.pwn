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
public OnTGMessage(TGBot:bot,const string[]){
    printf("[%d] : %s",_:bot,string);
}