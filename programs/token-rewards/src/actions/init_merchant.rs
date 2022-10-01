use crate::*;

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitMerchantParams {
    pub name: String,
    pub image: String,
}

#[derive(Accounts)]
#[instruction(params: InitMerchantParams)]
pub struct InitMerchant<'info> {
    #[account(
        init,
        seeds = [MERCHANT_SEED.as_bytes(), user.key().as_ref()],
        bump,
        payer = user,
        space = MerchantState::get_account_size(params)
    )]
    pub merchant: Account<'info, MerchantState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

// initialize new merchant account
impl InitMerchant<'_> {
    pub fn actuate(ctx: &mut Context<Self>, params: &InitMerchantParams) -> Result<()> {
        msg!("Initialize Merchant Account");

        let merchant = &mut ctx.accounts.merchant;
        merchant.user = ctx.accounts.user.key();
        merchant.promo_count = 0;
        merchant.name = params.name.to_string();
        merchant.image = params.image.to_string();

        Ok(())
    }
}
