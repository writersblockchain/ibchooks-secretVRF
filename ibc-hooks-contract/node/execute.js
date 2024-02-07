import { SecretNetworkClient, Wallet } from "secretjs";
import dotenv from "dotenv";
dotenv.config();

const wallet = new Wallet(process.env.MNEMONIC);

const secretjs = new SecretNetworkClient({
  chainId: "pulsar-3",
  url: "https://api.pulsar3.scrttestnet.com",
  wallet: wallet,
  walletAddress: wallet.address,
});

const hooksContractAddress = process.env.HOOKS_CONTRACT_ADDRESS;
const ibcChannelIdOnChain2 = "channel-3";

const RNGContractAddress = "secret15qra5alm78aut6ejlpv5uwje3qj33gtke52psk";
const RNGCodeHash =
  "d2da0e5562d7300803096c07f8f696e55f5d9ea11dc021e476fb0d01786a8907";

let execute = async () => {
  try {
    let tx = await secretjs.tx.ibc.transfer(
      {
        sender: wallet.address,
        receiver: hooksContractAddress,
        source_channel: ibcChannelIdOnChain2,
        source_port: "transfer",
        token: {
          amount: "1",
          denom: "uscrt", //the token you want to IBC in
        },
        timeout_timestamp: String(Math.floor(Date.now() / 1000) + 10 * 60), // 10 minutes
        memo: JSON.stringify({
          wasm: {
            contract: hooksContractAddress,
            msg: {
              request_random: {
                random_address: RNGContractAddress,
                random_code_hash: RNGCodeHash,
              },
            },
            ibc_callback: hooksContractAddress,
          },
        }),
      },
      {
        broadcastCheckIntervalMs: 100,
        gasLimit: 100_000,
        ibcTxsOptions: {
          resolveResponsesCheckIntervalMs: 250,
        },
      }
    );
    console.log("Transaction successful:", tx);
  } catch (error) {
    console.error("An error occurred:", error);
  }
};

execute();
