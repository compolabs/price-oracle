# Sway Land Price Oracle
The Sway Land Price Oracle receives price updates from a poster (who pulls the prices from coingecko). The price oracle verifies prices are within valid ranges, and if so, stores the prices so they can be accessed by Compound Money Market.

# How to deploy and use Swaygang's price oracle

Welcome to our tutorial on creating a price oracle in the Sway programming language for Fuel Network.
One of the key components in the blockchain is the price oracle, a decentralized system for providing accurate, up-to-date data on asset prices. Popular projects like Chainlink and Pyth.network have already proven the importance of price oracles in the blockchain ecosystem.

Fuel Network currently doesn't have any available price oracle solutions yet. That's why we would like to present a tutorial on building a price oracle in the Sway programming language, specifically designed for Fuel Network. This tutorial will guide you through the process of building a basic price oracle and provide you with the necessary code and steps to run it on a server using Docker. This oracle is not highly secure or scalable, but it will provide a good introduction to the concept of price oracles in the blockchain technology and give you the opportunity to get familiar with building solutions on Fuel Network. So, let's dive into the world of price oracles and see what it's all about!

## Deployment Steps
### Clone the Repository

To get started, you will need to clone the SwayGang price oracle repository from GitHub on your local computer. You can do this by using the git clone command in your terminal or command prompt:
```
git clone https://github.com/sway-gang/price-oracle.git
```

Once the cloning process is complete, navigate to the project folder by using the following command:

```
cd ./price-oracle
```

### Set Up the Environment
To set up the environment for the price oracle, you need to perform the following steps:

Change to the contract folder:
```
cd contract
```
Create a New Wallet (Optional):
You can use an existing wallet or create a new wallet to deploy the price oracle. If you already have a wallet and know how to obtain its secret key, you can skip this step and proceed to the next one.

The first step in setting up the environment is to create a new wallet. This wallet will be used to deploy the price oracle. You can create a new wallet using the following command:
```
forc-wallet new  
```
When you run the above command, you will be prompted to enter a password. This password is used to encrypt the secret key for your wallet, which is required to interact with Fuel Network.

After entering the password, you will see the following output:
```
Generating account with index: <INDEX>
Please enter your password to decrypt initialized wallet's phrases: <YOUR PASSWORD HERE>
Wallet address: fuel1...va0c2u
```

The Wallet address displayed in the output is the address of your newly created wallet.

Export the Secret Key:
The secret key is used to interact with Fuel Network and is required to deploy the price oracle. If you have created a new wallet, you can export its secret key using the following command:


```
forc-wallet export --account-index <YOUR INDEX HERE>
```
After entering your password, you will see the secret key for your account:

```
Please enter your password to decrypt initialized wallet's phrases: <YOUR PASSWORD HERE>
Secret key for account <INDEX>: 39a...5bd

### Press any key to complete. ###

```

Add the Secret Key to the .env File:
To make it easier to work with the price oracle, you can add the secret key to the .env file using the following command:
```
echo "SECRET=<YOUR SECRET HERE>" >> .env 
```

With these steps completed, you will now need to mint some ETH to pay the transaction fees when deploying the price oracle. You can obtain ETH from a faucet: 
https://faucet-beta-2.fuel.network/.

### Deploy and Initialize
In this step, we will be deploying and initializing the SwayGang price oracle. First, we will run the following command:

```
force build
```

This will compile the contract, and the output should be similar to:

```
  Compiled library "core".
  Compiled library "std".
  Compiled contract "oracle".
  Bytecode size is 4392 bytes.
```

To simplify the process, we've created a script that will deploy and initialize the oracle in the Tokyo test environment. You can run the script by executing the following command:
```
cargo test --package oracle --test integration_tests -- testnet_actions::deploy_and_initialize::deploy_and_initialize --exact --nocapture 
```

This will deploy and initialize the oracle and the output should be similar to:
```
running 1 test
✅ Initialize
✅ Oracle contract deployed
Hash:   0xd3ebf0eff0eda379b8b3eeb79c2f662d3d1b60110547b1590e6411e5a5f340df
Bech32: fuel1604lpmlsak3hnw9na6mectmx9573kcq3q4rmzkgwvsg7tf0ngr0sdrgqvs
test testnet_actions::deploy_and_initialize::deploy_and_initialize ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 27.23
```
Note: The owner of the oracle will be the address from which it was deployed. This means that you can only update the price from this address.


## Running the Oracle Update Server
Go to the server folder:
```
cd ../server
```

Set up the environment variables in the .env file:
```
echo "
SECRET=<YOUR_SECRET>#SECRET OF YOUR WALLET FROM THE LAST STEPS
ORACLE_ADDRESS=<YOUR_ORACLE_ADDRESS> #HASH ADDRESS OF ORACLE FROM THE LAST STEPS
FREQUENCY=60 #FREQUENCY OF UPDATE
" >> .env 
```

### Setup tokens list
You can manage the token list in the file `price-oracle/tokens.json`. By default `tokens.json` is filled by [swaylend](https://app.swaylend.com/) tokens.
You should provide info about each token in this format:
```
 {
   "asset_id": "0x0000000000000000000000000000000000000000000000000000000000000000",
   "symbol": "ETH",
   "coingeco_id": "ethereum",
   "default_price": 1200
}
```
You can find coingeco_id on the coingecko.com website in token info as API id. You can check an example here: https://www.coingecko.com/en/coins/ethereum

Start the server with the following command:
```
cargo run 
```
Upon successful execution, the following output should be displayed:

```
███████╗██╗    ██╗ █████╗ ██╗   ██╗     ██████╗  █████╗ ███╗   ██╗ ██████╗ 
██╔════╝██║    ██║██╔══██╗╚██╗ ██╔╝    ██╔════╝ ██╔══██╗████╗  ██║██╔════╝ 
███████╗██║ █╗ ██║███████║ ╚████╔╝     ██║  ███╗███████║██╔██╗ ██║██║  ███╗
╚════██║██║███╗██║██╔══██║  ╚██╔╝      ██║   ██║██╔══██║██║╚██╗██║██║   ██║
███████║╚███╔███╔╝██║  ██║   ██║       ╚██████╔╝██║  ██║██║ ╚████║╚██████╔╝
╚══════╝ ╚══╝╚══╝ ╚═╝  ╚═╝   ╚═╝        ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝                                                                         

✅ Oracle is alive

Oracle owner   = Address(330e6922460257edbaac821df96a555bc745041c97ff5411ec83f2b29f471c27)
Wallet address = Address(330e6922460257edbaac821df96a555bc745041c97ff5411ec83f2b29f471c27)
🪬 Price oracle update
1 USDC = $1.000435188
1 ETH = $1658.670603406
1 LINK = $7.16316701
1 BTC = $23025.231488235
1 UNI = $6.777685611
1 SWAY = $56.298952718
1 COMP = $56.298952718

⛽️ Gas used: 109362
⚖️ Balance: 0.499999991 ETH
👁 Oracle address: 0xd3ebf0eff0eda379b8b3eeb79c2f662d3d1b60110547b1590e6411e5a5f340df
-----------------------------------
```

You can also run the server using Docker by using the Dockerfile in the server directory. Here's how:

Build the Docker image:
```
sudo docker build --tag oracle .  
```

Run the Docker container:
```
sudo docker run --restart=always -d oracle   

```

## Summary
This tutorial on creating a price oracle in the Sway programming language for Fuel Network will guide you through the steps of building a basic, yet functional oracle. As a key component of blockchain infrastructure, price oracles provide accurate, up-to-date data on asset prices and are essential to the functioning of blockchain ecosystems. With this tutorial, you will gain hands-on experience building a solution for Fuel Network, and a better understanding of process of development on Fuel Network. 
