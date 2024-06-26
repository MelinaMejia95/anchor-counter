use anchor_lang::prelude::*;

declare_id!("8NGX2ja4eSPUjJfjJjwEi1J54JjVR1vWxUdgqSHdPVvP");

#[program]
mod counter {
    use super::*; // We're telling the program it has access to everything outside of it

    // Instructions 
    pub fn create_counter(ctx: Context<Create>, count: u64) -> Result<()> { // Result: Returns if everything worked or if it failed
        ctx.accounts.counter.number = count;
        ctx.accounts.counter.authority = ctx.accounts.authority.key();
        msg!("Counter created with number {}", count);
        Ok(()) // Always the last line in an instruction in anchor
    }

    pub fn delete_counter(_ctx: Context<Delete>) -> Result<()> {
        msg!("Counter deleted");
        Ok(())
    }

    pub fn increment_counter(ctx: Context<Increment>) -> Result<()> {
      let counter = &mut ctx.accounts.counter; // Create a mutable reference to the counter account
      counter.number += 1; // Increase the counter's value
      msg!("Counter incremented: {}", counter.number);
      Ok(())
    }

    pub fn decrement_counter(ctx: Context<Decrement>) -> Result<()> {
      let counter = &mut ctx.accounts.counter;
      counter.number -= 1; 
      msg!("Counter decremented: {}", counter.number);
      Ok(())
    }

    pub fn update_counter(ctx: Context<Update>, number: u64) -> Result<()> {
      ctx.accounts.counter.number = number;
      msg!("Counter updated to: {}", number);
      Ok(())
    }
}

// List of accounts, this is the context of an instruction
#[derive(Accounts)] // Macro to create the context of an instruction
#[instruction(count: u64)] // anchor know we're sending attributes to the instruction that is different from the context/account
pub struct Create<'info> { // 'info: it's a lifetime, can have any name
    // space = 8 bytes for the discriminator + the size of your structure
    #[account(init, payer = authority, space = 8 + 8 + 32)] // Indicates that the counter has an account structure
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub authority: Signer<'info>, // The authority is mutable because it has to pay the rent for Counter Account

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Delete<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint = counter.authority == authority.key() @ ErrorCode::NotAuthorized, // We're asking if counter.authority is the same that signed the delete instruction
        close = authority
    )]
    pub counter: Account<'info, Counter>
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(number: u64)]
pub struct Update<'info> {
  #[account(mut, constraint = counter.authority == authority.key() @ ErrorCode::NotAuthorized)]
  pub counter: Account<'info, Counter>,
  pub authority: Signer<'info>,
}

#[account]
pub struct Counter {
    number: u64, // 8 bytes
    authority: Pubkey, // 32 bytes
}

#[error_code]
pub enum ErrorCode {
  #[msg("You're not authorized")]
  NotAuthorized,
}
