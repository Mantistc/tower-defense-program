//! you can test all this thing with localhost, just run the solana test validator
//! put in the .env the localhost values and you're good to go.

//! remember to airdrop some sol first. I didn't put directly in the code bcs on devnet fails a lot

import { createSolanaClient } from "gill";
import { SOL_RPC } from "./constants";
import {
  InitializePlayerInstruction,
  UpdatePlayerInstruction,
} from "./instructions";
import { delay, getSigner } from "./utils";
import { buildAndSendTx } from "./transactions";

// init the rpc
const { rpc, sendAndConfirmTransaction } = createSolanaClient({
  urlOrMoniker: SOL_RPC,
});

const lastTimePlayed = Math.floor(new Date().getTime() / 1000);

async function initializePlayerTest() {
  const signer = await getSigner();

  const initializePlayerIx = new InitializePlayerInstruction({
    lastTimePlayed: BigInt(lastTimePlayed),
    signer: signer.address,
  });
  await initializePlayerIx.make();

  const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

  await buildAndSendTx(
    signer,
    [initializePlayerIx],
    latestBlockhash,
    sendAndConfirmTransaction
  );
}

async function updatePlayerTest() {
  const signer = await getSigner();
  const updatePlayerIx = new UpdatePlayerInstruction({
    lastTimePlayed: BigInt(lastTimePlayed),
    signer: signer.address,
    waveCount: 15,
  });
  await updatePlayerIx.make();

  const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

  await buildAndSendTx(
    signer,
    [updatePlayerIx],
    latestBlockhash,
    sendAndConfirmTransaction
  );
}

async function run() {
  await initializePlayerTest();
  await delay(5000);
  await updatePlayerTest();
}

// start the test
run();
