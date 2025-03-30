import {
  Address,
  getAddressEncoder,
  getProgramDerivedAddress,
  getUtf8Encoder,
} from "gill";
import { TOWER_DEFENSE_PROGRAM_ID } from "../constants";

export const PLAYER_SEED: string = "player";

export async function getPlayerAddress(signer: Address) {
  const seedEncoder = getUtf8Encoder();
  const addressEncoder = getAddressEncoder();
  const [player, bump] = await getProgramDerivedAddress({
    programAddress: TOWER_DEFENSE_PROGRAM_ID,
    seeds: [seedEncoder.encode(PLAYER_SEED), addressEncoder.encode(signer)],
  });
  return { player, bump, encodedPlayer: addressEncoder.encode(player) };
}
