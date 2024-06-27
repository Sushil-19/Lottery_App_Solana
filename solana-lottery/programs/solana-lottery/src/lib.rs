
use anchor_lang::prelude::*;
use solana_program::clock::Clock;
use solana_program::sysvar::Sysvar;
use solana_program::program_error::ProgramError;
use rand::seq::SliceRandom; // Import the SliceRandom trait
use rand::thread_rng; // Import the thread_rng function

declare_id!("EjCqXxWCvZTgRxXdacAPUZ5YVgjH7vphTmAkAK9NpvLZ");

#[program]
mod solana_lottery {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, prize_amount: u64) -> Result<()> {
        let lottery = &mut ctx.accounts.lottery;
        lottery.prize_amount = prize_amount;
        Ok(())
    }

    pub fn enter(ctx: Context<Enter>) -> Result<()> {
        let lottery = &mut ctx.accounts.lottery;
        lottery.entries.push(ctx.accounts.participant.key());
        Ok(())
    }

    pub fn draw(ctx: Context<Draw>) -> Result<()> {
        let lottery = &mut ctx.accounts.lottery;
        let clock = Clock::get().map_err(|_| ErrorCode::ClockError)?;

        if clock.unix_timestamp % 2 == 0 {
            let mut rng = thread_rng();
            if let Some(winner) = lottery.entries.choose(&mut rng) {
                lottery.winner = Some(*winner);
            } else {
                return Err(ErrorCode::NoEntries.into());
            }
        } else {
            return Err(ErrorCode::InvalidTimestamp.into());
        }

        Ok(())
    }
}

#[account]
pub struct Lottery {
    pub prize_amount: u64,
    pub entries: Vec<Pubkey>,
    pub winner: Option<Pubkey>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8 + 32 + 1)]
    pub lottery: Account<'info, Lottery>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Enter<'info> {
    #[account(mut)]
    pub lottery: Account<'info, Lottery>,
    #[account(mut)]
    pub participant: Signer<'info>,
}

#[derive(Accounts)]
pub struct Draw<'info> {
    #[account(mut)]
    pub lottery: Account<'info, Lottery>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("An error occurred while accessing the clock.")]
    ClockError,
    #[msg("There are no entries in the lottery.")]
    NoEntries,
    #[msg("Invalid timestamp.")]
    InvalidTimestamp,
}
