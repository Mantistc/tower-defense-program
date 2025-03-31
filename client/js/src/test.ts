//! you can test all this thing with localhost, just run the solana test validator
//! put in the .env the localhost values and you're good to go.

//! remember to airdrop some sol first. I didn't put directly in the code bcs on devnet fails a lot

import { createSolanaClient, generateKeyPairSigner } from "gill";
import { SOL_RPC } from "./constants";
import {
  InitializePlayerInstruction,
  UpdatePlayerGameValues,
  UpdatePlayerAuthority,
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

async function updatePlayerGameValuesTest() {
  const signer = await getSigner();
  const updatePlayerIx = new UpdatePlayerGameValues({
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

async function updatePlayerAuthorityTest() {
  const signer = await getSigner();
  const newAuthority = (await generateKeyPairSigner()).address;
  const updatePlayerAuthority = new UpdatePlayerAuthority({
    signer: signer.address,
    newAuthority,
  });
  await updatePlayerAuthority.make();

  const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

  await buildAndSendTx(
    signer,
    [updatePlayerAuthority],
    latestBlockhash,
    sendAndConfirmTransaction
  );
}

async function run() {
  await initializePlayerTest();
  await delay(5000);
  await updatePlayerGameValuesTest();
  await delay(5000);
  await updatePlayerAuthorityTest();
}

// start the test
run();
