use near_contract_standards::non_fungible_token::metadata::{NFTContractMetadata, TokenMetadata};
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::AccountId;

pub trait OwnerAction {
    fn set_owner(&mut self, owner: AccountId);

    fn set_contract_metadata(&mut self, metadata: NFTContractMetadata) -> bool;

    fn batch_mint_nfts(
        &mut self,
        base_metadata: TokenMetadata,
        owner_and_tokens: Vec<(AccountId, TokenMetadata)>,
    );
}
