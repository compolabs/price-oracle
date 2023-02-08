# Sway Land Price Oracle
The Sway Land Price Oracle receives price updates from a poster (who pulls the prices from coingeco). The price oracle verifies the prices are within valid ranges, and if so, stores the prices so they can be accessed by the Compound Money Market.

# How to deploy and use swaygang price oracle

## Deploy
### Clone repository

This step involves cloning the price oracle repository from GitHub onto your local computer. To do this, you will need to use the `git clone` command.

`git clone https://github.com/sway-gang/price-oracle.git`

After that let’s go to the project folder

`cd ./price-oracle`

### Setup env
```
cd contract
```

Creation of a new wallet
```
forc-wallet new  
```

Output
```
Generating account with index: <INDEX>
Please enter your password to decrypt initialized wallet's phrases: <YOUR PASSWORD HERE>
Wallet address: fuel1...va0c2u
```

Exporting of secret
```
forc-wallet export --account-index <YOUR INDEX HERE>
```
Output
```
Please enter your password to decrypt initialized wallet's phrases: <YOUR PASSWORD HERE>
Secret key for account <INDEX>: 39a...5bd

### Press any key to complete. ###

```

Adding secret to the .`env` file
```
echo "SECRET=<YOUR SECRET HERE>" >> .env 
```

After those steps, you should mint some `ETH` to pay the tx fee
https://faucet-beta-2.fuel.network/

### Deploy and initialize

```
force build
```
Output
```
  Compiled library "core".
  Compiled library "std".
  Compiled contract "oracle".
  Bytecode size is 4392 bytes.
```

To make it simple I made a deploy and initialized script using the Tokyo test environment
```
cargo test --package oracle --test integration_tests -- testnet_actions::deploy_and_initialize::deploy_and_initialize --exact --nocapture 
```

Output
```
running 1 test
✅ Initialize
✅ Oracle contract deployed
Hash:   0xd3ebf0eff0eda379b8b3eeb79c2f662d3d1b60110547b1590e6411e5a5f340df
Bech32: fuel1604lpmlsak3hnw9na6mectmx9573kcq3q4rmzkgwvsg7tf0ngr0sdrgqvs
test testnet_actions::deploy_and_initialize::deploy_and_initialize ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 27.23
```

## Run oracle update server
Go to the server folder
```
cd ../server
```

Setup of юenv
```
echo "
SECRET=<YOUR_SECRET>#SECRET OF YOUR WALLET FROM THE LAST STEPS
ORACLE_ADDRESS=<YOUR_ORACLE_ADDRESS> #HASH ADDRESS OF ORACLE FROM THE LAST STEPS
FREQUENCY=60 #FREQUENCY OF UPDATE
" >> .env 
```

Let's start our server
```
cargo run 
```
Output
```


███████╗██╗    ██╗ █████╗ ██╗   ██╗     ██████╗  █████╗ ███╗   ██╗ ██████╗ 
██╔════╝██║    ██║██╔══██╗╚██╗ ██╔╝    ██╔════╝ ██╔══██╗████╗  ██║██╔════╝ 
███████╗██║ █╗ ██║███████║ ╚████╔╝     ██║  ███╗███████║██╔██╗ ██║██║  ███╗
╚════██║██║███╗██║██╔══██║  ╚██╔╝      ██║   ██║██╔══██║██║╚██╗██║██║   ██║
███████║╚███╔███╔╝██║  ██║   ██║       ╚██████╔╝██║  ██║██║ ╚████║╚██████╔╝
╚══════╝ ╚══╝╚══╝ ╚═╝  ╚═╝   ╚═╝        ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝                                                                         

✅ Oracle is alive

🪬 Price oracle update
1 USDC = $0.999918042
1 ETH = $1657.97964672
1 LINK = $7.159493594
1 BTC = $22993.081278477
1 UNI = $6.776005727
1 SWAY = $55.802529211
1 COMP = $55.802529211

⛽️ Gas used: 109362
⚖️ Balance: 0.999999963 ETH
👁 Oracle address: 0xd3ebf0eff0eda379b8b3eeb79c2f662d3d1b60110547b1590e6411e5a5f340df
-----------------------------------
```