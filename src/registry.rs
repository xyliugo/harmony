use crate::encoding::FormattingToken;
use std::collections::HashMap;

/// Registry for Harmony formatting tokens
/// This module manages the registration and lookup of all formatting tokens
pub struct TokenRegistry {
    tokens: HashMap<FormattingToken, String>,
}

impl TokenRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            tokens: HashMap::new(),
        };

        // Register all formatting tokens
        // Basic block control tokens
        registry.register(FormattingToken::StartOfBlock, "<|begin_of_text|>");
        registry.register(FormattingToken::EndOfBlock, "<|end_of_text|>");

        // Text header tokens
        registry.register(FormattingToken::StartText, "<|start_header_id|>");
        registry.register(FormattingToken::EndText, "<|end_header_id|>");

        // Meta tokens - Critical additions for proper meta handling
        registry.register(FormattingToken::MetaSep, "<|meta_sep|>");
        registry.register(FormattingToken::MetaEnd, "<|meta_end|>");

        // Channel tokens
        registry.register(FormattingToken::Channel, "<|channel|>");

        // Role-based tokens
        registry.register(FormattingToken::System, "<|system|>");
        registry.register(FormattingToken::User, "<|user|>");
        registry.register(FormattingToken::Assistant, "<|assistant|>");

        registry
    }

    /// Register a token mapping
    pub fn register(&mut self, token: FormattingToken, value: &str) {
        self.tokens.insert(token, value.to_string());
    }

    /// Get the string representation of a token
    pub fn get_token(&self, token: &FormattingToken) -> Option<&String> {
        self.tokens.get(token)
    }

    /// Get a token by its string representation
    pub fn get_token_from_string(&self, s: &str) -> Option<FormattingToken> {
        for (token, value) in self.tokens.iter() {
            if value == s {
                return Some(*token);
            }
        }
        None
    }

    /// Check if a token is registered
    pub fn is_registered(&self, token: &FormattingToken) -> bool {
        self.tokens.contains_key(token)
    }

    /// Get all registered tokens
    pub fn get_all_tokens(&self) -> &HashMap<FormattingToken, String> {
        &self.tokens
    }

    /// Get all token values
    pub fn get_all_values(&self) -> Vec<&String> {
        self.tokens.values().collect()
    }
}

impl Default for TokenRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for TokenRegistry {
    fn eq(&self, other: &Self) -> bool {
        self.tokens == other.tokens
    }
}

impl Eq for TokenRegistry {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meta_sep_registration() {
        let registry = TokenRegistry::new();
        assert!(registry.is_registered(&FormattingToken::MetaSep));
        assert_eq!(registry.get_token(&FormattingToken::MetaSep), Some(&"<|meta_sep|>".to_string()));
    }

    #[test]
    fn test_meta_end_registration() {
        let registry = TokenRegistry::new();
        assert!(registry.is_registered(&FormattingToken::MetaEnd));
        assert_eq!(registry.get_token(&FormattingToken::MetaEnd), Some(&"<|meta_end|>".to_string()));
    }

    #[test]
    fn test_get_token_from_string() {
        let registry = TokenRegistry::new();
        assert_eq!(registry.get_token_from_string("<|meta_sep|>"), Some(FormattingToken::MetaSep));
        assert_eq!(registry.get_token_from_string("<|meta_end|>"), Some(FormattingToken::MetaEnd));
    }

    #[test]
    fn test_all_tokens_registered() {
        let registry = TokenRegistry::new();
        
        let required_tokens = vec![
            FormattingToken::StartOfBlock,
            FormattingToken::EndOfBlock,
            FormattingToken::StartText,
            FormattingToken::EndText,
            FormattingToken::MetaSep,
            FormattingToken::MetaEnd,
            FormattingToken::Channel,
            FormattingToken::System,
            FormattingToken::User,
            FormattingToken::Assistant,
        ];

        for token in required_tokens {
            assert!(registry.is_registered(&token), "Token {:?} should be registered", token);
        }
    }
}
