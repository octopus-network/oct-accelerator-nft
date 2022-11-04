mod activity;
mod impls;
mod interfaces;
mod types;

use crate::types::{ActivityCreatorId, ActivityId};
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata,
};
use near_contract_standards::non_fungible_token::{NonFungibleToken, Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet, Vector};
use near_sdk::json_types::U128;
use near_sdk::Promise;
use near_sdk::{log, AccountId, PromiseOrValue};
use near_sdk::{near_bindgen, BorshStorageKey, PanicOnDefault};

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
    CreatorWhitelist,
    Activities,
    ActivityTokens,
    TokenActivity,
    ActivitiesByCreators,
    ActivitiesByCreator { creator_id: ActivityCreatorId },
    ActivityTokenMetadata,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner: AccountId,
    activity_id: ActivityId,
    token: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    creator_whitelist: UnorderedSet<ActivityCreatorId>,
    activities: UnorderedMap<ActivityId, ActivityCreatorId>,
    activity_tokens: LookupMap<ActivityId, UnorderedSet<TokenId>>,
    token_activity: LookupMap<TokenId, ActivityId>,
    activities_by_creators: LookupMap<ActivityCreatorId, Vector<ActivityId>>,
    activity_token_metadata: LookupMap<ActivityId, TokenMetadata>,
}

near_contract_standards::impl_non_fungible_token_approval!(Contract, token);

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        let contract = Self {
            owner: owner_id.clone(),
            activity_id: 0,
            token: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            creator_whitelist: UnorderedSet::new(StorageKey::CreatorWhitelist),
            activities: UnorderedMap::new(StorageKey::Activities),
            activity_tokens: LookupMap::new(StorageKey::ActivityTokens),
            token_activity: LookupMap::new(StorageKey::TokenActivity),
            activities_by_creators: LookupMap::new(StorageKey::ActivitiesByCreators),
            activity_token_metadata: LookupMap::new(StorageKey::ActivityTokenMetadata),
        };
        contract
    }

    pub(crate) fn assign_activity_id(&mut self) -> ActivityId {
        self.activity_id += 1;
        self.activity_id
    }

    pub(crate) fn internal_get_nft_metadata(&self, token_id: &TokenId) -> Option<TokenMetadata> {
        self.token_activity
            .get(&token_id)
            // if token exist then use activity token metadata as token's metadata
            .and_then(|activity_id| self.activity_token_metadata.get(&activity_id))
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_contract_standards::non_fungible_token::core::NonFungibleTokenCore;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;
    use std::collections::HashMap;

    use super::*;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn sample_token_metadata() -> TokenMetadata {
        TokenMetadata {
            title: Some("Olympus Mons".into()),
            description: Some("The tallest mountain in the charted solar system".into()),
            media: None,
            media_hash: None,
            copies: Some(1u64),
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        }
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new(
            accounts(1).into(),
            NFTContractMetadata {
                spec: "".to_string(),
                name: "".to_string(),
                symbol: "".to_string(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        );
        testing_env!(context.is_view(true).build());
        let token = contract.nft_token("tokentoken".to_string());
        println!("{:?}", token);
        // assert_eq!(contract.nft_token("1".to_string()), None);
    }
}
