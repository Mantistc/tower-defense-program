use crate::states::{load_mut_unchecked, Player, PLAYER_SEED};
use core::mem::size_of;
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_system::instructions::CreateAccount;

#[inline(always)]
pub fn process_initialize_player(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
    program_id: &Pubkey,
) -> ProgramResult {
    let [player, signer, _remaining @ ..] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !signer.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let last_time_played = u64::from_le_bytes(
        instruction_data[0..8]
            .try_into()
            .map_err(|_error| ProgramError::InvalidInstructionData)?,
    );

    let space = size_of::<Player>();

    let lamports = Rent::get()?.minimum_balance(space);

    let bump = [instruction_data[8]];

    let seeds = &[
        Seed::from(PLAYER_SEED),
        Seed::from(signer.key().as_ref()),
        Seed::from(&bump),
    ];

    let pda_signer = [Signer::from(seeds)];
    let _account_creation = CreateAccount {
        from: signer,
        to: player,
        lamports,
        space: space as u64,
        owner: program_id,
    }
    .invoke_signed(&pda_signer)?;

    let player = unsafe { load_mut_unchecked::<Player>(player.borrow_mut_data_unchecked())? };

    player.set_values(last_time_played, 0);
    player.set_authority(signer.key());

    Ok(())
}
