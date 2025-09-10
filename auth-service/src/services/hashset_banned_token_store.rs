use std::collections::HashSet;

use crate::domain::BannedTokenStore;

#[derive(Default)]
pub struct HashsetBannedTokenStore {
    banned_tokens: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn add_token(&mut self, token: String) {
        self.banned_tokens.insert(token);
    }

    async fn is_token_banned(&self, token: &str) -> bool {
        self.banned_tokens.contains(token)
    }
}
