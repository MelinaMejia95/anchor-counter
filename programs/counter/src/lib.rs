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

    pub fn increase_counter(ctx: Context<Increase>, increment: u64) -> Result<()> {
      let counter = &mut ctx.accounts.counter; // Create a mutable reference to the counter account
      counter.number += increment; // Increase the counter's value
      msg!("Counter incremented in {}, result: {}", increment, counter.number);
      Ok(())
    }
}

// List of accounts, this is the context of an instruction
#[derive(Accounts)] // Macro to create the context of an instruction
pub struct Create<'info> { // 'info: it's a lifetime, can have any name
    // space = 8 bytes for the discriminator + the size of your structure
    #[account(init, payer = authority, space = 8 + 8 + 32)] // Le indica a contador que es una estructura account y le da ciertos atributos.
    pub counter: Account<'info, Counter>,

    #[account(mut)] // mut: Indica que va a cambiar
    pub authority: Signer<'info>, // La autoridad es variable porque tiene que pagar la renta de la cuenta contador

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Delete<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint = counter.authority == counter.key(), // Estamos preguntando si el contador.autoridad es el mismo que firmó al hacer el borrar
        close = authority
    )]
    pub counter: Account<'info, Counter>
}

#[derive(Accounts)]
pub struct Increase<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
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
