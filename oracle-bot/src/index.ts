import TelegramService from "./services/telegramService";
import { sleep } from "./utils/utils";

const { log, groupMessage } = new TelegramService();

//For normal bot life, the account must have enough money to pay commissions
//The bot will keep its funds in USDT, respectively, there should be enough of them on the account

const blackList = ["AbunLGErT5ctzVN8MVjb4Ad9YgjpubB8Hqb17VxzfAck"];

// user says "im selling 1 PUZZLE for 18 USDN"
// token0 = PUZZLE, token1 = USDN
// amount0 = 1 * 10**8, amount1 = 18 * 10**6
(async () => {
  while (true) {
    await sleep(1000);
  }
})();

process.stdout.write("Bot has been started ✅ ");
