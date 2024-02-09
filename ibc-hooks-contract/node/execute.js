import { SecretNetworkClient, Wallet } from "secretjs";
import dotenv from "dotenv";
dotenv.config();

const wallet = new Wallet(process.env.MNEMONIC, {bech32Prefix: "axelar"} );

const secretjs = new SecretNetworkClient({
  chainId: "axelar-testnet-lisbon-3",
  url: "https://lcd-axelar-testnet.imperator.co:443",
  wallet: wallet,
  walletAddress: wallet.address,
});

const hooksContractAddress = "secret1jgzgf5zxf82zj846qm74ak7ggfkc6pmx2wa9fe";
const ibcChannelIdOnChain2 = "channel-311";

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
          denom: "ibc/898741B7C169F5E82640382F30DDF19DEE029683B93D679ABC35DCF0458E1E38", //the token you want to IBC in
        },
        timeout_timestamp: String(Math.floor(Date.now() / 1000) + 10 * 60), // 10 minutes

        memo: JSON.stringify({
          wasm: {
            contract: hooksContractAddress,
            msg: {
              request_random: {
                job_id: "1",
                num_words: "20",
                callback_channel_id: "channel-3",
                callback_to_address: "axelar1j7n3xx4sfgjea4unghd78qvnvxdz49cxag5pf0",
                timeout_sec_from_now: "900"
              },
            },
          },
        }),
      },
      {
        broadcastCheckIntervalMs: 100,
        gasLimit: 100_000,
        ibcTxsOptions: {
          resolveResponsesCheckIntervalMs: 250,
        },
        feeDenom: "uaxl",
        gasLimit: 400000
      }
    );
    console.log("Transaction successful:", tx);
  } catch (error) {
    console.error("An error occurred:", error);
  }
};

execute();
