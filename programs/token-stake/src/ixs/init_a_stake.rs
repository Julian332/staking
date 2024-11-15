use crate::{Deposit, Initialize};
use anchor_lang::prelude::*;
use std::ops::DerefMut;

//todo
pub const POOL_STATE_SPACE: usize = 8  + 8;
#[account]
pub struct StakingState {
    // pub staking_token_ata: Pubkey,
    // pub staking_token: Pubkey,
    pub total_lp: u64,
}

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    // msg!("Greetings from: {:?}", ctx.program_id);
    // let state = ctx.accounts.pool_state.deref_mut();
    // msg!(&format!("++++++{},",state.total_lp));
    // state.staking_token_ata = ctx.accounts.staking_token_ata.key();
    // state.staking_token = ctx.accounts.staking_token.key();
    // state.total_lp = 0;
    Ok(())
}
