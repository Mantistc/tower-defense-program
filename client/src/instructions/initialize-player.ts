import {
  IAccountLookupMeta,
  IAccountMeta,
  IInstruction,
} from "@solana/instructions";
import { ReadonlyUint8Array, Address, AccountRole, getU64Encoder } from "gill";
import { getPlayerAddress } from "../accounts";
import { SYSTEM_PROGRAM_ADDRESS } from "@solana-program/system";
import { TOWER_DEFENSE_PROGRAM_ID } from "../constants";
import { InstructionDiscriminator } from ".";

export class InitializePlayerInstruction implements IInstruction {
  accounts:
    | readonly (IAccountLookupMeta<string, string> | IAccountMeta<string>)[]
    | undefined;

  data: ReadonlyUint8Array | undefined;

  programAddress: Address<string> = TOWER_DEFENSE_PROGRAM_ID;

  constructor() {}

  public async ix(signer: Address, lastTimePlayed: bigint) {
    const { player, bump } = await getPlayerAddress(signer);

    const u64Encoder = getU64Encoder();
    const lastTimePlayedEncoded = u64Encoder.encode(lastTimePlayed);

    const allBytes = [
      InstructionDiscriminator.InitializePlayer,
      ...lastTimePlayedEncoded,
      bump,
    ];

    const data: ReadonlyUint8Array = Uint8Array.from(allBytes);

    const accounts = [
      { address: player, addressIndex: 0, role: AccountRole.WRITABLE },
      {
        address: signer,
        addressIndex: 1,
        role: AccountRole.WRITABLE_SIGNER,
      },
      {
        address: SYSTEM_PROGRAM_ADDRESS,
        addressIndex: 2,
        role: AccountRole.READONLY,
      },
    ];
    this.accounts = accounts;
    this.data = data;
  }
}
