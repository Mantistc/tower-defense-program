import {
  IAccountLookupMeta,
  IAccountMeta,
  IInstruction,
} from "@solana/instructions";
import { ReadonlyUint8Array, Address, AccountRole } from "gill";
import { getPlayerAddress } from "../accounts";
import { SYSTEM_PROGRAM_ADDRESS } from "@solana-program/system";
import { TOWER_DEFENSE_PROGRAM_ID } from "../constants";
import { InstructionDiscriminator } from ".";

export interface InitializePlayerInstructionArgs {
  signer: Address;
}

export class InitializePlayerInstruction implements IInstruction {
  accounts:
    | readonly (IAccountLookupMeta<string, string> | IAccountMeta<string>)[]
    | undefined;

  data: ReadonlyUint8Array | undefined;

  programAddress: Address<string> = TOWER_DEFENSE_PROGRAM_ID;

  args: InitializePlayerInstructionArgs;

  constructor(args: InitializePlayerInstructionArgs) {
    this.args = args;
  }

  public async make() {
    const { signer } = this.args;
    const { player, bump } = await getPlayerAddress(signer);

    const allBytes = [InstructionDiscriminator.InitializePlayer, bump];

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
