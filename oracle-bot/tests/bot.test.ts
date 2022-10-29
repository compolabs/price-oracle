import { OracleAbi__factory } from "../src/contracts";
import { SEED } from "../src/config";
import { Wallet } from "fuels";
import axios from "axios";
import BN from "../src/utils/BN";
const CONTRACT_ID =
  "0x7cb52c0c6f43fceaac13ade73d7ee8531dea944320fa5038c7133939fe3f079b";
const wallet = new Wallet(
  SEED,
  "https://node-beta-1.fuel.network/graphql"
);
const contract = OracleAbi__factory.connect(CONTRACT_ID, wallet);

describe("Eth price oracle TEST", () => {
  it("update prices", async () => {
    const { data } = await axios.get(
      "https://api.coingecko.com/api/v3/simple/price?ids=ethereum%2Cdai&vs_currencies=usd&include_market_cap=false&include_24hr_vol=false&include_24hr_change=false&include_last_updated_at=false&precision=18"
    );
    await contract.functions.set_price(
      BN.parseUnits(data.ethereum.usd, 18).toString()
    );
  });
});
