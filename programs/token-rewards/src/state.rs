use crate::*;

// Merchant account state
// TODO: add "giftcard" token discount/reward basis point here
#[account]
pub struct MerchantState {
    pub user: Pubkey,     // 32
    pub mint: Pubkey,     // 32
    pub mint_bump: u8,    // 1
    pub promo_count: u64, // 8
    pub image: String,    // 4 + len()
    pub name: String,     // 4 + len()
}

impl MerchantState {
    pub fn get_account_size(params: InitMerchantParams) -> usize {
        return 8 + 32 + 32 + 1 + 8 + 4 + params.name.len() + 4 + params.image.len();
    }
}

// Data for each "coupon" promotion
#[account]
pub struct PromoState {
    pub user: Pubkey, // 32
    pub mint: Pubkey, // 32
    pub bump: u8,     // 1
}
