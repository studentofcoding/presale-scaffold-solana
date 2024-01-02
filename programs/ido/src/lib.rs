use anchor_lang::prelude::*;

declare_id!("DoK1DNxEDNA69r9849ZkDuoJt1Pz39rZ5qwgaqPykLXH");

#[program]
pub mod ido {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
