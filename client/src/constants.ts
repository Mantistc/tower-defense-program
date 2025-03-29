import { Address } from "gill";

export const TOWER_DEFENSE_PROGRAM_ID: Address =
  "tdpUmm2N1bhmSfYAynVuWWFWSd5aF5LmiBTPXJEwoW6" as Address<"tdpUmm2N1bhmSfYAynVuWWFWSd5aF5LmiBTPXJEwoW6">;

export const SOL_RPC: string =
  process.env.SOL_RPC || "https://api.devnet.solana.com";

export const TEST_KEYPAIR: string | undefined = process.env.TEST_KEYPAIR;
