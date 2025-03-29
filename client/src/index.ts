import {
  createKeyPairFromBytes,
  createSolanaClient,
  createTransaction,
  getAddressFromPublicKey,
  getExplorerLink,
  getSignatureFromTransaction,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
} from "gill";
import { SOL_RPC, TEST_KEYPAIR } from "./constants";
import { InitializePlayerInstruction } from "./instructions";

async function initializePlayerTest() {
  // init the rpc
  const { rpc, sendAndConfirmTransaction } = createSolanaClient({
    urlOrMoniker: SOL_RPC,
  });

  if (!TEST_KEYPAIR) {
    throw new Error("A keypair is required in the .env file, anon.");
  }

  // init the signer (this must have sol to sign and initialize the player account)
  const secretKeyArray = Uint8Array.from(JSON.parse(TEST_KEYPAIR));
  const keypair = await createKeyPairFromBytes(secretKeyArray);
  const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

  // simulate last time played
  const lastTimePlayed = new Date().getTime() / 1000; //   /1000 to get secs

  const initializePlayerIx = new InitializePlayerInstruction(
    await getAddressFromPublicKey(keypair.publicKey),
    BigInt(lastTimePlayed)
  );

  const transaction = createTransaction({
    version: 0,
    feePayer: keypair,
    instructions: [initializePlayerIx],
  });

  const signedTransaction = await signTransactionMessageWithSigners(
    setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, transaction)
  );

  const signature: string = getSignatureFromTransaction(signedTransaction);

  console.log(getExplorerLink({ transaction: signature }));

  await sendAndConfirmTransaction(signedTransaction);
}

// start the test
initializePlayerTest();
