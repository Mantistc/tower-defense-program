import {
  IAccountLookupMeta,
  IAccountMeta,
  IInstruction,
} from "@solana/instructions";
import { ReadonlyUint8Array, Address, AccountRole, getU64Encoder } from "gill";
import { getPlayerAddress } from "../accounts";
import { TOWER_DEFENSE_PROGRAM_ID } from "../constants";
import { InstructionDiscriminator } from ".";

export interface UpdatePlayerInstructionArgs {
  signer: Address;
  lastTimePlayed: bigint;
  waveCount: number;
}

export class UpdatePlayerInstruction implements IInstruction {
  accounts:
    | readonly (IAccountLookupMeta<string, string> | IAccountMeta<string>)[]
    | undefined;

  data: ReadonlyUint8Array | undefined;

  programAddress: Address<string> = TOWER_DEFENSE_PROGRAM_ID;

  args: UpdatePlayerInstructionArgs;

  constructor(args: UpdatePlayerInstructionArgs) {
    this.args = args;
  }

  public async make() {
    const { signer, lastTimePlayed, waveCount } = this.args;
    const { player } = await getPlayerAddress(signer);

    const u64Encoder = getU64Encoder();
    const lastTimePlayedEncoded = u64Encoder.encode(lastTimePlayed);

    const allBytes = [
      InstructionDiscriminator.UpdatePlayer,
      ...lastTimePlayedEncoded,
      waveCount,
    ];

    const data: ReadonlyUint8Array = Uint8Array.from(allBytes);

    const accounts = [
      { address: player, addressIndex: 0, role: AccountRole.WRITABLE },
      {
        address: signer,
        addressIndex: 1,
        role: AccountRole.WRITABLE_SIGNER,
      },
    ];
    this.accounts = accounts;
    this.data = data;
  }
}
