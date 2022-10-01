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
        seeds = [REWARD_SEED.as_bytes(), merchant.key().as_ref()],
        bump = merchant.mint_bump
    )]
    pub reward_mint: Account<'info, Mint>,

    #[account(
        // address = USDC_MINT_ADDRESS.parse::<Pubkey>().unwrap(),
    )]
    pub usdc_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = customer,
        associated_token::mint = reward_mint,
        associated_token::authority = receiver
    )]
    pub receiver_reward_token: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        constraint = customer_usdc_token.mint == usdc_mint.key(),
        constraint = customer_usdc_token.owner == customer.key()
    )]
    pub customer_usdc_token: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        constraint = merchant_usdc_token.mint == usdc_mint.key(),
        constraint = merchant_usdc_token.owner == user.key()
    )]
    pub merchant_usdc_token: Box<Account<'info, TokenAccount>>,

    /// CHECK:
    #[account(mut)]
    pub user: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub receiver: AccountInfo<'info>,

    #[account(mut)]
    pub customer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

// distribute "giftcard" token by minting, in exchange for transfer of another token
impl MintReward<'_> {
    pub fn actuate(ctx: &mut Context<Self>, params: &MintRewardParams) -> Result<()> {
        let merchant = ctx.accounts.merchant.key();

        let seeds = &[
            REWARD_SEED.as_bytes(),
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
                to: ctx.accounts.merchant_usdc_token.to_account_info(),
            },
        );
        token::transfer(cpi_ctx, params.amount)?;

        msg!("Minting Tokens");
        let cpi_accounts = MintTo {
            mint: ctx.accounts.reward_mint.to_account_info(),
            to: ctx.accounts.receiver_reward_token.to_account_info(),
            authority: ctx.accounts.reward_mint.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer);

        token::mint_to(cpi_ctx, params.amount)?;
        msg!("Token Minted");

        Ok(())
    }
}
