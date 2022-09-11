use crate::*;

#[derive(Accounts)]
pub struct MintPromo<'info> {
    pub promo: Account<'info, PromoState>,

    #[account(mut,
        seeds = [MINT_SEED, promo.key().as_ref()],
        bump = promo.bump
    )]
    pub promo_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,

    #[account(mut,
        constraint = token_account.mint == promo_mint.key(),
        constraint = token_account.owner == user.key()
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
}

// distribute "coupon" token by minting
impl MintPromo<'_> {
    pub fn actuate(ctx: &mut Context<Self>) -> Result<()> {
        let promo = ctx.accounts.promo.key();

        let seeds = &[MINT_SEED, promo.as_ref(), &[ctx.accounts.promo.bump]];
        let signer = [&seeds[..]];

        msg!("Minting Promo");
        let cpi_accounts = MintTo {
            mint: ctx.accounts.promo_mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.promo_mint.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer);

        token::mint_to(cpi_ctx, 1)?;
        msg!("Token Minted");

        Ok(())
    }
}
