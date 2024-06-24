use anchor_lang::prelude::*;

declare_id!("8NGX2ja4eSPUjJfjJjwEi1J54JjVR1vWxUdgqSHdPVvP");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
