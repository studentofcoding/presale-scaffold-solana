use anchor_lang::prelude::*;
pub mod instructions;

use instructions::*;

declare_id!("DoK1DNxEDNA69r9849ZkDuoJt1Pz39rZ5qwgaqPykLXH");

#[program]
pub mod ido {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    // pub fn buy_token(ctx: Context<BuyToken>, amount: u64) -> Result<()> {
    //     buy_token::handler(ctx, amount)
    // }
}

#[derive(Accounts)]
pub struct Initialize {}
