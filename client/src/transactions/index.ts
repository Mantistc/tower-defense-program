import {
  IInstruction,
  SendAndConfirmTransactionWithSignersFunction,
  TransactionBlockhashLifetime,
  TransactionSigner,
  createTransaction,
  getExplorerLink,
  getSignatureFromTransaction,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
} from "gill";
import { CLUSTER } from "../constants";

export async function buildAndSendTx(
  feePayer: TransactionSigner,
  instructions: IInstruction[],
  lifeTimes: TransactionBlockhashLifetime,
  sendTx: SendAndConfirmTransactionWithSignersFunction
) {
  const transaction = createTransaction({
    version: 0,
    feePayer,
    instructions,
  });

  const signedTransaction = await signTransactionMessageWithSigners(
    setTransactionMessageLifetimeUsingBlockhash(lifeTimes, transaction)
  );

  const signature: string = getSignatureFromTransaction(signedTransaction);

  console.log(getExplorerLink({ transaction: signature, cluster: CLUSTER }));

  await sendTx(signedTransaction);
}
