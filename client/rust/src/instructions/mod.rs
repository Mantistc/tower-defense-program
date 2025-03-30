pub mod initialize_player;

use std::mem::MaybeUninit;

pub use initialize_player::*;


const UNINIT_BYTE: MaybeUninit<u8> = MaybeUninit::<u8>::uninit();

#[inline(always)]
fn write_bytes(destination: &mut [MaybeUninit<u8>], source: &[u8]) {
    for (d, s) in destination.iter_mut().zip(source.iter()) {
        d.write(*s);
    }
}


#[repr(u8)]
pub enum InstructionDiscriminator{
    InitializePlayer,
    UpdatePlayerGameValues,
    UpdatePlayerAuthority
}
