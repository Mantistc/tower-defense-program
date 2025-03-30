import {
  createSignerFromKeyPair,
  // createKeyPairSignerFromBytes,
  createSolanaClient,
  createTransaction,
  getExplorerLink,
  getSignatureFromTransaction,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
} from "gill";
import { SOL_RPC } from "./constants";
import { InitializePlayerInstruction } from "./instructions";
import { loadKeypairFromEnvironment } from "gill/node";

async function initializePlayerTest() {
  // init the rpc
  const { rpc, sendAndConfirmTransaction } = createSolanaClient({
    urlOrMoniker: SOL_RPC,
  });

  // init the signer (this must have sol to sign and initialize the player account)
  const keypair = await loadKeypairFromEnvironment("TEST_KEYPAIR");
  const signer = await createSignerFromKeyPair(keypair);
  const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

  // simulate last time played
  const lastTimePlayed = Math.floor(new Date().getTime() / 1000); //   /1000 to get secs

  const initializePlayerIx = new InitializePlayerInstruction();
  await initializePlayerIx.ix(signer.address, BigInt(lastTimePlayed));

  console.log(initializePlayerIx);

  const transaction = createTransaction({
    version: 0,
    feePayer: signer,
    instructions: [initializePlayerIx],
  });

  const signedTransaction = await signTransactionMessageWithSigners(
    setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, transaction)
  );

  const signature: string = getSignatureFromTransaction(signedTransaction);

  console.log(getExplorerLink({ transaction: signature, cluster: "devnet" }));

  await sendAndConfirmTransaction(signedTransaction, {
    skipPreflight: true,
    commitment: "confirmed",
  });
}

// start the test
initializePlayerTest();
