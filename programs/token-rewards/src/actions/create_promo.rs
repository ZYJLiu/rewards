use crate::*;

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CreatePromoParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(Accounts)]
pub struct CreatePromo<'info> {
    #[account(mut,
     constraint = merchant.user == user.key())]
    pub merchant: Account<'info, MerchantState>,

    #[account(
        init,
        seeds = [merchant.key().as_ref(), merchant.promo_count.to_be_bytes().as_ref()],
        bump,
        payer = user,
        space = 8 + std::mem::size_of::<PromoState>()
    )]
    pub promo: Account<'info, PromoState>,

    #[account(
        init,
        seeds = [MINT_SEED, promo.key().as_ref()],
        bump,
        payer = user,
        mint::decimals = 0,
        mint::authority = promo_mint,

    )]
    pub promo_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,

    /// CHECK: test
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    #[account(address = mpl_token_metadata::id())]
    pub token_metadata_program: Program<'info, TokenMetadata>,
}

impl CreatePromo<'_> {
    pub fn actuate(ctx: &mut Context<Self>, params: &CreatePromoParams) -> Result<()> {
        msg!("Create Promo");

        msg!("Create Promo Metadata");
        let promo_data_key = ctx.accounts.promo.key();

        let seeds = &[
            MINT_SEED,
            promo_data_key.as_ref(),
            &[*ctx.bumps.get("promo_mint").unwrap()],
        ];

        let signer = [&seeds[..]];

        let account_info = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.promo_mint.to_account_info(),
            ctx.accounts.promo_mint.to_account_info(),
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
                ctx.accounts.promo_mint.key(),
                ctx.accounts.promo_mint.key(),
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

        let promo = &mut ctx.accounts.promo;
        promo.user = ctx.accounts.user.key();
        promo.mint = ctx.accounts.promo_mint.key();
        promo.bump = *ctx.bumps.get("promo_mint").unwrap();

        msg!("Update Merchant Promo Count");
        let merchant = &mut ctx.accounts.merchant;
        merchant.promo_count = merchant.promo_count.checked_add(1).unwrap();

        Ok(())
    }
}
