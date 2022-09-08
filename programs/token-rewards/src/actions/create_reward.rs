use crate::*;

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CreateRewardParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(Accounts)]
pub struct CreateReward<'info> {
    #[account(mut,
     constraint = merchant.user == user.key())]
    pub merchant: Account<'info, MerchantState>,

    #[account(
        init,
        seeds = [REWARD_SEED, merchant.key().as_ref()],
        bump,
        payer = user,
        mint::decimals = 6,
        mint::authority = reward_mint,

    )]
    pub reward_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,

    /// CHECK: test
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    /// CHECK: test
    pub token_metadata_program: AccountInfo<'info>,
}

impl CreateReward<'_> {
    pub fn actuate(ctx: &mut Context<Self>, params: &CreateRewardParams) -> Result<()> {
        msg!("Create Reward Token");
        let merchant = ctx.accounts.merchant.key();

        let seeds = &[
            &REWARD_SEED,
            merchant.as_ref(),
            &[*ctx.bumps.get("reward_mint").unwrap()],
        ];

        let signer = [&seeds[..]];

        let account_info = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.reward_mint.to_account_info(),
            ctx.accounts.reward_mint.to_account_info(),
            ctx.accounts.user.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        invoke_signed(
            &create_metadata_accounts_v2(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.reward_mint.key(),
                ctx.accounts.reward_mint.key(),
                ctx.accounts.user.key(),
                ctx.accounts.user.key(), // todo: change to PDA
                params.name.to_string(),
                params.symbol.to_string(),
                params.uri.to_string(),
                None,
                0,
                true,
                true,
                None,
                None,
            ),
            account_info.as_slice(),
            &signer,
        )?;

        msg!("Update Merchant Promo Count");
        let merchant = &mut ctx.accounts.merchant;
        merchant.mint = ctx.accounts.reward_mint.key();
        merchant.mint_bump = *ctx.bumps.get("reward_mint").unwrap();

        Ok(())
    }
}
