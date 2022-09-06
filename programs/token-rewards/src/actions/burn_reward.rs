use crate::*;

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct BurnRewardParams {
    pub amount: u64,
}

#[derive(Accounts)]
pub struct BurnReward<'info> {
    #[account(mut,
     constraint = merchant.user == user.key())]
    pub merchant: Account<'info, MerchantState>,

    #[account(mut,
        seeds = [REWARD_SEED],
        bump = merchant.mint_bump
    )]
    pub reward_mint: Account<'info, Mint>,

    // #[account(
    //     address = USDC_MINT_ADDRESS.parse::<Pubkey>().unwrap(),
    // )]
    // pub usdc_mint: Account<'info, Mint>,
    #[account(mut,
        constraint = customer_reward_token.mint == reward_mint.key(),
        constraint = customer_reward_token.owner == customer.key()
    )]
    pub customer_reward_token: Account<'info, TokenAccount>,

    // #[account(mut,
    //     constraint = customer_usdc_token.mint == usdc_mint.key(),
    //     constraint = customer_usdc_token.owner == customer.key()
    // )]
    // pub customer_usdc_token: Account<'info, TokenAccount>,

    // #[account(mut,
    //     constraint = user_usdc_token.mint == usdc_mint.key(),
    //     constraint = user_usdc_token.owner == user.key()
    // )]
    // pub user_usdc_token: Account<'info, TokenAccount>,
    /// CHECK:
    #[account(mut)]
    pub user: AccountInfo<'info>,

    #[account(mut)]
    pub customer: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

impl BurnReward<'_> {
    pub fn actuate(ctx: &mut Context<Self>, params: &BurnRewardParams) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Burn {
                mint: ctx.accounts.reward_mint.to_account_info(),
                from: ctx.accounts.customer_reward_token.to_account_info(),
                authority: ctx.accounts.customer.to_account_info(),
            },
        );
        token::burn(cpi_ctx, params.amount)?;

        Ok(())
    }
}
