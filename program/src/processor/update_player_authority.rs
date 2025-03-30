use crate::states::{load_mut_unchecked, Player};
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

#[inline(always)]
pub fn process_update_player_authority(accounts: &[AccountInfo]) -> ProgramResult {
    let [player, signer, new_authority, _remaining @ ..] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !signer.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if new_authority.key() == signer.key() {
        return Err(ProgramError::InvalidAccountData);
    }

    let player = unsafe { load_mut_unchecked::<Player>(player.borrow_mut_data_unchecked())? };

    if *signer.key() != player.authority {
        return Err(ProgramError::IncorrectAuthority);
    }

    player.set_authority(new_authority.key());
    Ok(())
}
