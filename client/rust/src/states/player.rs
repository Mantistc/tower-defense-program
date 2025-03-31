use solana_sdk::program_error::ProgramError;

#[repr(C)]
pub struct Player {
    pub wave_reached: u8,
    pub last_played: [u8; 8],
    pub authority: [u8; 32],
}

impl Transmutable for Player {
    const LEN: usize = 1 + 8 + 32;
}

pub trait Transmutable {
    /// The length of the type.
    ///
    /// This must be equal to the size of each individual field in the type.
    const LEN: usize;
}

impl Player {
    pub fn unpack(data: &[u8]) -> Result<Self, ProgramError> {
        if data.len() < Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }

        let wave_reached = data[0];
        let last_played = u64::from_le_bytes(
            data[1..9]
                .try_into()
                .map_err(|_error| ProgramError::InvalidAccountData)?,
        );
        let authority: [u8; 32] = data[9..41]
            .try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?;

        Ok(Player {
            wave_reached,
            last_played: last_played.to_le_bytes(),
            authority,
        })
    }
}
