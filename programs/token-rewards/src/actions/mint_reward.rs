use crate::*;

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct MintRewardParams {
    pub amount: u64,
}

#[derive(Accounts)]
pub struct MintReward<'info> {
    #[account(mut,
     constraint = merchant.user == user.key())]
    pub merchant: Account<'info, MerchantState>,

    #[account(mut,
        seeds = [REWARD_SEED, merchant.key().as_ref()],
        bump = merchant.mint_bump
    )]
    pub reward_mint: Account<'info, Mint>,

    #[account(
        address = USDC_MINT_ADDRESS.parse::<Pubkey>().unwrap(),
    )]
    pub usdc_mint: Account<'info, Mint>,

    #[account(mut,
        constraint = customer_reward_token.mint == reward_mint.key(),
        constraint = customer_reward_token.owner == customer.key()
    )]
    pub customer_reward_token: Account<'info, TokenAccount>,

    #[account(mut,
        constraint = customer_usdc_token.mint == usdc_mint.key(),
        constraint = customer_usdc_token.owner == customer.key()
    )]
    pub customer_usdc_token: Account<'info, TokenAccount>,

    #[account(mut,
        constraint = user_usdc_token.mint == usdc_mint.key(),
        constraint = user_usdc_token.owner == user.key()
    )]
    pub user_usdc_token: Account<'info, TokenAccount>,

    /// CHECK:
    #[account(mut)]
    pub user: AccountInfo<'info>,

    #[account(mut)]
    pub customer: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

impl MintReward<'_> {
    pub fn actuate(ctx: &mut Context<Self>, params: &MintRewardParams) -> Result<()> {
        let merchant = ctx.accounts.merchant.key();

        let seeds = &[
            REWARD_SEED,
            merchant.as_ref(),
            &[ctx.accounts.merchant.mint_bump],
        ];
        let signer = [&seeds[..]];

        msg!("Transfer Tokens");
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.customer_usdc_token.to_account_info(),
                authority: ctx.accounts.customer.to_account_info(),
                to: ctx.accounts.user_usdc_token.to_account_info(),
            },
        );
        token::transfer(cpi_ctx, params.amount)?;

        msg!("Minting Tokens");
        let cpi_accounts = MintTo {
            mint: ctx.accounts.reward_mint.to_account_info(),
            to: ctx.accounts.customer_reward_token.to_account_info(),
            authority: ctx.accounts.reward_mint.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer);

        token::mint_to(cpi_ctx, params.amount)?;
        msg!("Token Minted");

        Ok(())
    }
}
