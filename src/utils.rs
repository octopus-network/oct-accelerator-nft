use near_contract_standards::non_fungible_token::metadata::TokenMetadata;

pub fn merge_token_metadata(
    base_metadata: &TokenMetadata,
    extra_metadata: &TokenMetadata,
) -> TokenMetadata {
    TokenMetadata {
        title: extra_metadata.title.clone().or(base_metadata.title.clone()),
        description: extra_metadata
            .description
            .clone()
            .or(base_metadata.description.clone()),
        media: extra_metadata.media.clone().or(base_metadata.media.clone()),
        media_hash: extra_metadata
            .media_hash
            .clone()
            .or(base_metadata.media_hash.clone()),
        copies: extra_metadata
            .copies
            .clone()
            .or(base_metadata.copies.clone()),
        issued_at: extra_metadata
            .issued_at
            .clone()
            .or(base_metadata.issued_at.clone()),
        expires_at: extra_metadata
            .expires_at
            .clone()
            .or(base_metadata.expires_at.clone()),
        starts_at: extra_metadata
            .starts_at
            .clone()
            .or(base_metadata.starts_at.clone()),
        updated_at: extra_metadata
            .updated_at
            .clone()
            .or(base_metadata.updated_at.clone()),
        extra: extra_metadata.extra.clone().or(base_metadata.extra.clone()),
        reference: extra_metadata
            .reference
            .clone()
            .or(base_metadata.reference.clone()),
        reference_hash: extra_metadata
            .reference_hash
            .clone()
            .or(base_metadata.reference_hash.clone()),
    }
}
