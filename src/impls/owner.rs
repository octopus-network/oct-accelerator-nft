use crate::interfaces::OwnerAction;
use crate::utils::merge_token_metadata;
use crate::*;

#[near_bindgen]
impl OwnerAction for Contract {
    fn set_owner(&mut self, owner: AccountId) {
        self.assert_owner();
        self.owner = owner;
    }

    fn set_contract_metadata(&mut self, metadata: NFTContractMetadata) -> bool {
        self.assert_owner();
        self.metadata.set(&metadata)
    }

    #[payable]
    fn batch_mint_nfts(
        &mut self,
        base_metadata: TokenMetadata,
        owner_and_tokens: Vec<(AccountId, TokenMetadata)>,
    ) {
        self.assert_owner();

        for owner_and_token in owner_and_tokens {
            let nft_owner_id = owner_and_token.0;
            let token_id = format!("{}:{}", env::block_timestamp(), nft_owner_id).to_string();

            self.token.internal_mint(
                token_id.clone(),
                nft_owner_id,
                Some(merge_token_metadata(&base_metadata, &owner_and_token.1)),
            );
        }
    }
}
