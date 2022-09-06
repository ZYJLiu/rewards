use crate::*;

#[account]
pub struct MerchantState {
    pub user: Pubkey,     // 32
    pub mint: Pubkey,     // 32
    pub mint_bump: u8,    // 1
    pub promo_count: u64, // 8
    pub image: String,    // 4 + len()
    pub name: String,     // 4 + len()
}

#[account]
pub struct PromoState {
    pub user: Pubkey, // 32
    pub mint: Pubkey, // 32
    pub bump: u8,     // 1
}