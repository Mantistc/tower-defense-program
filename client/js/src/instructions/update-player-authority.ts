import {
  IAccountLookupMeta,
  IAccountMeta,
  IInstruction,
} from "@solana/instructions";
import { ReadonlyUint8Array, Address, AccountRole } from "gill";
import { getPlayerAddress } from "../accounts";
import { TOWER_DEFENSE_PROGRAM_ID } from "../constants";
import { InstructionDiscriminator } from ".";

export interface UpdatePlayerAuthorityInstructionArgs {
  signer: Address;
  newAuthority: Address;
}

export class UpdatePlayerAuthority implements IInstruction {
  accounts:
    | readonly (IAccountLookupMeta<string, string> | IAccountMeta<string>)[]
    | undefined;

  data: ReadonlyUint8Array | undefined;

  programAddress: Address<string> = TOWER_DEFENSE_PROGRAM_ID;

  args: UpdatePlayerAuthorityInstructionArgs;

  constructor(args: UpdatePlayerAuthorityInstructionArgs) {
    this.args = args;
  }

  public async make() {
    const { signer, newAuthority } = this.args;
    const { player } = await getPlayerAddress(signer);

    const allBytes = [InstructionDiscriminator.UpdatePlayerAuthority];

    const data: ReadonlyUint8Array = Uint8Array.from(allBytes);

    const accounts = [
      { address: player, addressIndex: 0, role: AccountRole.WRITABLE },
      {
        address: signer,
        addressIndex: 1,
        role: AccountRole.WRITABLE_SIGNER,
      },
      {
        address: newAuthority,
        addressIndex: 2,
        role: AccountRole.READONLY,
      },
    ];
    this.accounts = accounts;
    this.data = data;
  }
}
