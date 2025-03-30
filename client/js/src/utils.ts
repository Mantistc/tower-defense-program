import { createSignerFromKeyPair } from "gill";
import { loadKeypairFromEnvironment } from "gill/node";

export function delay(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export async function getSigner() {
  const keypair = await loadKeypairFromEnvironment("TEST_KEYPAIR");
  const signer = await createSignerFromKeyPair(keypair);
  return signer;
}
