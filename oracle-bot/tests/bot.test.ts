import {OracleAbi__factory} from "../src/contracts";
import {SEED} from "../src/config";
import {Wallet} from "fuels";
import axios from "axios";
import BN from "../src/utils/BN";
import {FunctionInvocationScope} from "@fuel-ts/contract";

const CONTRACT_ID =
    "0xe5afff6b7dbc66d082d3e8d3abd9074e3a220196815a5d3f53e9aaf81fadb3a7";
const wallet = new Wallet(
    SEED,
    "https://node-beta-1.fuel.network/graphql"
);
const contract = OracleAbi__factory.connect(CONTRACT_ID, wallet);

const decimal = 8

describe("Eth price oracle TEST", () => {
    // it("hardcode test", async () => {
    //     // // const ethTx = new FunctionInvocationScope(contract, "set_price_eth", 1650);
    //     // contract.multiCall([
    //     //     new FunctionInvocationScope(contract, 'set_price_eth', 1650),
    //     //     new FunctionInvocationScope(contract, 'set_price_dai', 1)
    //     // ])
    //     const result = await Promise.all([
    //         contract.functions.set_price_eth("1650").txParams({gasPrice: 1}).call(),
    //         contract.functions.set_price_dai('1').txParams({gasPrice: 1}).call()
    //     ])
    //     console.log(result)
    // });
    it("update prices", async () => {
        const {data} = await axios.get(
            "https://api.coingecko.com/api/v3/simple/price?ids=ethereum%2Cdai&vs_currencies=usd&include_market_cap=false&include_24hr_vol=false&include_24hr_change=false&include_last_updated_at=false&precision=" + decimal
        );
        await Promise.all([
            contract.functions.set_price_eth(BN.parseUnits(data.ethereum.usd, decimal).toString()).txParams({gasPrice: 1}).call(),
            contract.functions.set_price_dai(BN.parseUnits(data.dai.usd, decimal).toString()).txParams({gasPrice: 1}).call()
        ])
    });
    it("read prices", async () => {
        const {value: eth} = await contract.functions.price_eth().get();
        const {value: dai} = await contract.functions.price_dai().get();
        console.log({eth: eth.toString(), dai: dai.toString()})
    }, 50000000);
});
