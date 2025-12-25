use anchor_lang::prelude::*;

declare_id!("DzsNMUqVhpyj6rznbph4jjDKTshPaKdWVHYHDBETYgXE");

#[program]
pub mod GrokMultiverse { 
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.is_registered = true;
        user_account.points = 200; 
        msg!("User registered successfully with 200 points!");
        Ok(())
    }

    pub fn add_referral(ctx: Context<AddReferral>) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.points += 50; 
        msg!("Referral added! Current points: {}", user_account.points);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 1 + 8)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddReferral<'info> {
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
}

#[account]
pub struct UserAccount {
    pub is_registered: bool,
    pub points: u64,
}
