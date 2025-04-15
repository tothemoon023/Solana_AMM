//declare_id!("2TcTgLFmEpVWTH5arJrfitEj93SBLe3tTRRKNdCeeA8p");
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

mod errors;
mod state;
mod instructions;

use instructions::*;

declare_id!("2TcTgLFmEpVWTH5arJrfitEj93SBLe3tTRRKNdCeeA8p");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: u64, fee: u16, authority: Option<Pubkey>) -> Result<()> {
        ctx.accounts.init(seed, fee, authority, ctx.bumps)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        ctx.accounts.withdraw(amount, max_x, max_y)
    }

    pub fn swap(ctx: Context<Swap>, is_x: bool, amount_in: u64, min_amount_out: u64) -> Result<()> {
        ctx.accounts.swap(is_x, amount_in, min_amount_out)
    }
}

/* malikathaiyab@Malikas-MacBook-Air amm % anchor build
.
.
   Compiling amm v0.1.0 (/Users/malikathaiyab/solana_workspace/Q2builder_cohort/amm/programs/amm)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 16.47s
     Running unittests src/lib.rs (/Users/malikathaiyab/solana_workspace/Q2builder_cohort/amm/target/debug/deps/amm-334e93c0878bc989)
malikathaiyab@Malikas-MacBook-Air amm % */