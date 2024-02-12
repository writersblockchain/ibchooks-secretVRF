import { json_to_bytes, hex_to_bytes } from "@blake.regalia/belt";
import {
  Wallet,
  broadcast_result,
  create_and_sign_tx_direct,
} from "@solar-republic/neutrino";
import { encodeGoogleProtobufAny } from "@solar-republic/cosmos-grpc/google/protobuf/any";
import { queryCosmosBankAllBalances } from "@solar-republic/cosmos-grpc/cosmos/bank/v1beta1/query";
import {
  SI_MESSAGE_TYPE_COSMWASM_WASM_MSG_EXECUTE_CONTRACT,
  encodeCosmwasmWasmMsgExecuteContract,
} from "@solar-republic/cosmos-grpc/cosmwasm/wasm/v1/tx";
import dotenv from "dotenv";
dotenv.config();

let execute = async () => {
  const privateKey = hex_to_bytes(process.env.PRIVATE_KEY);
  const lcdUrl = "https://lcd-juno.whispernode.com:443";
  const rpcUrl = "https://rpc-juno-ia.cosmosia.notional.ventures/";

  const junoWallet = await Wallet(privateKey, "juno-1", lcdUrl, rpcUrl);

  //   console.log(junoWallet);

  const [httpResponse, resultText, resultStruct] =
    await queryCosmosBankAllBalances(junoWallet.lcd, junoWallet.addr);

  if (
    !resultStruct ||
    !resultStruct.balances ||
    !resultStruct.balances.length
  ) {
    throw Error(`Account ${junoWallet.addr} has no balances`);
  }

  const message = encodeGoogleProtobufAny(
    SI_MESSAGE_TYPE_COSMWASM_WASM_MSG_EXECUTE_CONTRACT,
    encodeCosmwasmWasmMsgExecuteContract(
      junoWallet.addr,
      process.env.CONSUMER_CONTRACT,
      json_to_bytes({
        request_random: {
          job_id: "10",
        },
      })
    )
  );

  const gasLimit = 150_000n; // BigInt is a part of modern JavaScript
  const gasAmount = Math.ceil(Number(gasLimit) * 0.125);

  const [txRawBytes, signDoc, txHash] = await create_and_sign_tx_direct(
    junoWallet,
    [message],
    [[`${gasAmount}`, "ujuno"]],
    `${gasLimit}`
  );

  const [errorCode, responseText, result] = await broadcast_result(
    junoWallet,
    txRawBytes,
    txHash
  );

  if (result) {
    console.log(
      `Gas used: ${result.result ? result.result.gas_used : "unknown"}`
    );
  }
};
execute();
