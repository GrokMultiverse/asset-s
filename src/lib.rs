use anchor_lang::prelude::*;

// Your Program ID
declare_id!("DzsNMUqVhpyj6rznbph4jjDKTshPaKdWVHYHDBETYgXE");

#[program]
pub mod grokmultiverse {
    use super::*;

    // 1. Initial Account Creation (200 Points for Wallet Connect)
    pub fn initialize_account(ctx: Context<InitializeAccount>) -> Result<()> {
        let user_stats = &mut ctx.accounts.user_stats;
        user_stats.owner = ctx.accounts.user.key();
        user_stats.points = 200; 
        user_stats.total_referred = 0;
        user_stats.x_followed = false;
        user_stats.tg_joined = false;
        
        msg!("GrokMultiverse: Account Created! 200 Points awarded.");
        Ok(())
    }

    // 2. X (Twitter) Follow Task (100 Points)
    pub fn claim_x_points(ctx: Context<UpdateTasks>) -> Result<()> {
        let user_stats = &mut ctx.accounts.user_stats;
        require!(!user_stats.x_followed, ErrorCode::AlreadyClaimed);
        
        user_stats.points += 100;
        user_stats.x_followed = true;
        msg!("GrokMultiverse: 100 Points added for X Follow!");
        Ok(())
    }

    // 3. Telegram Join Task (100 Points)
    pub fn claim_tg_points(ctx: Context<UpdateTasks>) -> Result<()> {
        let user_stats = &mut ctx.accounts.user_stats;
        require!(!user_stats.tg_joined, ErrorCode::AlreadyClaimed);
        
        user_stats.points += 100;
        user_stats.tg_joined = true;
        msg!("GrokMultiverse: 100 Points added for Telegram Join!");
        Ok(())
    }

    // 4. Referral System (50 Points)
    pub fn process_referral(ctx: Context<ProcessReferral>) -> Result<()> {
        let referrer_stats = &mut ctx.accounts.referrer_stats;
        referrer_stats.points += 50;
        referrer_stats.total_referred += 1;
        
        msg!("GrokMultiverse: 50 points awarded to referrer.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeAccount<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8 + 8 + 1 + 1 + 40, 
        seeds = [b"user-stats", user.key().as_ref()],
        bump
    )]
    pub user_stats: Account<'info, UserAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTasks<'info> {
    #[account(mut, seeds = [b"user-stats", user.key().as_ref()], bump)]
    pub user_stats: Account<'info, UserAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct ProcessReferral<'info> {
    #[account(
        mut, 
        seeds = [b"user-stats", referrer.key().as_ref()], 
        bump
    )]
    pub referrer_stats: Account<'info, UserAccount>,
    /// CHECK: This is the referrer's public key
    pub referrer: UncheckedAccount<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct UserAccount {
    pub owner: Pubkey,
    pub points: u64,
    pub total_referred: u64,
    pub x_followed: bool,
    pub tg_joined: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("You have already claimed this task reward.")]
    AlreadyClaimed,
}
