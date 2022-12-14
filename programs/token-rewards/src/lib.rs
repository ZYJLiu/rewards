use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, MintTo, Token, TokenAccount},
};
use mpl_token_metadata::instruction::create_metadata_accounts_v2;
use mpl_token_metadata::ID as mpl_token_metadata;

pub mod actions;
pub mod state;
use actions::*;
use state::*;

declare_id!("3wHwBiegty6hFBYSeEjS4c4wuBkeeCfHU8JN4SsHjEoC");

#[program]
pub mod token_rewards {
    use super::*;

    // initialize merchant account
    pub fn init_merchant(mut ctx: Context<InitMerchant>, params: InitMerchantParams) -> Result<()> {
        InitMerchant::actuate(&mut ctx, &params)
    }

    // create new "coupon" mint
    pub fn create_promo(mut ctx: Context<CreatePromo>, params: CreatePromoParams) -> Result<()> {
        CreatePromo::actuate(&mut ctx, &params)
    }

    // mint "coupon"
    pub fn mint_promo(mut ctx: Context<MintPromo>) -> Result<()> {
        MintPromo::actuate(&mut ctx)
    }

    // create merchant "giftcard" token
    pub fn create_reward(mut ctx: Context<CreateReward>, params: CreateRewardParams) -> Result<()> {
        CreateReward::actuate(&mut ctx, &params)
    }

    // mint "giftcard" tokens in exchange for "USDC" token
    pub fn mint_reward(mut ctx: Context<MintReward>, params: MintRewardParams) -> Result<()> {
        MintReward::actuate(&mut ctx, &params)
    }

    // burn "giftcard" tokens, TODO: add optional two token input & logic to handle
    pub fn burn_reward(mut ctx: Context<BurnReward>, params: BurnRewardParams) -> Result<()> {
        BurnReward::actuate(&mut ctx, &params)
    }
}

// const USDC_MINT_ADDRESS: &str = "Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr";

const MERCHANT_SEED: &str = "MERCHANT";
const MINT_SEED: &str = "MINT";
const REWARD_SEED: &str = "REWARD";

// const MERCHANT_SEED: &[u8] = b"MERCHANT";
// const MINT_SEED: &[u8] = b"MINT";
// const REWARD_SEED: &[u8] = b"REWARD";

#[derive(Clone)]
pub struct TokenMetadata;
impl anchor_lang::Id for TokenMetadata {
    fn id() -> Pubkey {
        mpl_token_metadata
    }
}
