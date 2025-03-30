use crate::states::Player;
use core::mem::size_of;
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_log::log;
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
    log!("starting!");
    if !signer.is_signer() {
        return Err(ProgramError::IncorrectAuthority);
    }
    log!("ix data len: {}", instruction_data.len());
    let last_time_played = u64::from_le_bytes(
        instruction_data[0..8]
            .try_into()
            .map_err(|_error| ProgramError::InvalidInstructionData)?,
    );
    log!("last time!!");

    let space = size_of::<Player>();

    let lamports = Rent::get()?.minimum_balance(space);
    log!("lamports !!");

    let _account_creation = CreateAccount {
        from: signer,
        to: player,
        lamports,
        space: space as u64,
        owner: program_id,
    }
    .invoke()?;
    log!("account creation !!");

    let player_data = unsafe { player.borrow_mut_data_unchecked() };
    let wave_count = 0u8;
    log!("data borrow !!");

    player_data[0..1].copy_from_slice(&wave_count.to_le_bytes());
    player_data[1..9].copy_from_slice(&last_time_played.to_le_bytes());
    log!("write !!");

    Ok(())
}
