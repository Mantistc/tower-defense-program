use pinocchio::pubkey::Pubkey;
use super::Transmutable;

#[repr(C)]
#[derive(Default)]
pub struct Player {
    pub wave_reached: u8,
    pub last_played: u64,
    pub authority: [u8; 32],
}

impl Transmutable for Player {
    const LEN: usize = core::mem::size_of::<Player>();
}

pub const PLAYER_SEED: &'static [u8] = b"player";
pub const MAX_POSSIBLE_WAVE_COUNT: u8 = 30;

impl Player {
    #[inline(always)]
    pub fn set_authority(&mut self, authority: &Pubkey) {
        self.authority = *authority;
    }

    #[inline(always)]
    pub fn set_values(&mut self, last_played: u64, wave_reached: u8) {
        self.last_played = last_played;
        self.wave_reached = wave_reached;
    }
}
