use crate::*;

#[derive(Accounts)]
pub struct MintPromo<'info> {
    pub promo: Account<'info, PromoState>,
    #[account(mut,
        seeds = [MINT_SEED.as_bytes(), promo.key().as_ref()],
        bump = promo.bump
    )]
    pub promo_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = promo_mint,
        associated_token::authority = user
    )]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

// distribute "coupon" token by minting
impl MintPromo<'_> {
    pub fn actuate(ctx: &mut Context<Self>) -> Result<()> {
        let promo = ctx.accounts.promo.key();

        let seeds = &[
            MINT_SEED.as_bytes(),
            promo.as_ref(),
            &[ctx.accounts.promo.bump],
        ];
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
