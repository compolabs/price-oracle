import {sleep} from "./utils/utils";
import {Wallet} from "fuels";
import {SEED} from "./config";
import {OracleAbi__factory} from "./contracts";
import axios from "axios";
import BN from "./utils/BN";

// const { log, groupMessage } = new TelegramService();

const CONTRACT_ID =
    "0x7cb52c0c6f43fceaac13ade73d7ee8531dea944320fa5038c7133939fe3f079b";
const wallet = new Wallet(
    SEED,
    "https://node-beta-1.fuel.network/graphql"
);
const contract = OracleAbi__factory.connect(CONTRACT_ID, wallet);
const decimal = 8;

(async () => {
  while (true) {
    const {data} = await axios.get(
        "https://api.coingecko.com/api/v3/simple/price?ids=ethereum%2Cdai&vs_currencies=usd&include_market_cap=false&include_24hr_vol=false&include_24hr_change=false&include_last_updated_at=false&precision=" + decimal
    );
    await Promise.all([
      contract.functions.set_price_eth(BN.parseUnits(data.ethereum.usd, decimal).toString()).txParams({gasPrice: 1}).call(),
      contract.functions.set_price_dai(BN.parseUnits(data.dai.usd, decimal).toString()).txParams({gasPrice: 1}).call()
    ])
    await sleep(1000);
  }
})();

process.stdout.write("Bot has been started ✅ ");
