use crate::states::{load_mut_unchecked, Player, MAX_POSSIBLE_WAVE_COUNT};
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

#[inline(always)]
pub fn process_update_player_game_values(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let [player, signer, _remaining @ ..] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !signer.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let player = unsafe { load_mut_unchecked::<Player>(player.borrow_mut_data_unchecked())? };

    if *signer.key() != player.authority {
        return Err(ProgramError::IncorrectAuthority);
    }

    let last_time_played = u64::from_le_bytes(
        instruction_data[0..8]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?,
    );

    let wave_reached = instruction_data[8];

    if wave_reached > MAX_POSSIBLE_WAVE_COUNT {
        return Err(ProgramError::InvalidInstructionData);
    }

    player.set_values(last_time_played, wave_reached);
    Ok(())
}
