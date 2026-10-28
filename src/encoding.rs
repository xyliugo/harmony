use crate::registry::HarmonyEncoding;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum FormattingToken {
    StartOfBlock,
    EndOfBlock,
    StartText,
    EndText,
    MetaSep,
    MetaEnd,
    Channel,
    System,
    User,
    Assistant,
}

impl FormattingToken {
    pub fn from_str(s: &str) -> Self {
        match s {
            "<|begin_of_text|>" => FormattingToken::StartOfBlock,
            "<|end_of_text|>" => FormattingToken::EndOfBlock,
            "<|start_header_id|>" => FormattingToken::StartText,
            "<|end_header_id|>" => FormattingToken::EndText,
            "<|meta_sep|>" => FormattingToken::MetaSep, // Fixed: was incorrectly "<|channel|>"
            "<|meta_end|>" => FormattingToken::MetaEnd,
            "<|channel|>" => FormattingToken::Channel,
            "<|system|>" => FormattingToken::System,
            "<|user|>" => FormattingToken::User,
            "<|assistant|>" => FormattingToken::Assistant,
            _ => FormattingToken::StartOfBlock,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            FormattingToken::StartOfBlock => "<|begin_of_text|>",
            FormattingToken::EndOfBlock => "<|end_of_text|>",
            FormattingToken::StartText => "<|start_header_id|>",
            FormattingToken::EndText => "<|end_header_id|>",
            FormattingToken::MetaSep => "<|meta_sep|>", // Fixed: was incorrectly "<|channel|>"
            FormattingToken::MetaEnd => "<|meta_end|>",
            FormattingToken::Channel => "<|channel|>",
            FormattingToken::System => "<|system|>",
            FormattingToken::User => "<|user|>",
            FormattingToken::Assistant => "<|assistant|>",
        }
    }
}

#[derive(Debug, Clone)]
pub struct HarmonyEncoding {
    pub formatting_tokens: HashMap<FormattingToken, String>,
    pub pad_token: String,
}

impl HarmonyEncoding {
    pub fn new() -> Self {
        let mut formatting_tokens = HashMap::new();

        // Format tokens
        formatting_tokens.insert(FormattingToken::StartOfBlock, "<|begin_of_text|>".to_string());
        formatting_tokens.insert(FormattingToken::EndOfBlock, "<|end_of_text|>".to_string());
        formatting_tokens.insert(FormattingToken::StartText, "<|start_header_id|>".to_string());
        formatting_tokens.insert(FormattingToken::EndText, "<|end_header_id|>".to_string());
        formatting_tokens.insert(FormattingToken::MetaSep, "<|meta_sep|>".to_string()); // Fixed registration
        formatting_tokens.insert(FormattingToken::MetaEnd, "<|meta_end|>".to_string()); // Fixed registration
        formatting_tokens.insert(FormattingToken::Channel, "<|channel|>".to_string());
        formatting_tokens.insert(FormattingToken::System, "<|system|>".to_string());
        formatting_tokens.insert(FormattingToken::User, "<|user|>".to_string());
        formatting_tokens.insert(FormattingToken::Assistant, "<|assistant|>".to_string());

        Self {
            formatting_tokens,
            pad_token: "<|pad|>".to_string(),
        }
    }

    pub fn get_token_string(&self, token: &FormattingToken) -> String {
        self.formatting_tokens.get(token).cloned().unwrap_or("<|pad|>".to_string())
    }

    pub fn get_token(&self, token_string: &str) -> Option<&FormattingToken> {
        self.formatting_tokens.iter().find(|(_, s)| s == token_string).map(|(t, _)| t)
    }
}

impl Default for HarmonyEncoding {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formatting_token_from_str() {
        assert_eq!(FormattingToken::from_str("<|begin_of_text|>"), FormattingToken::StartOfBlock);
        assert_eq!(FormattingToken::from_str("<|meta_sep|>"), FormattingToken::MetaSep);
        assert_eq!(FormattingToken::from_str("<|meta_end|>"), FormattingToken::MetaEnd);
    }

    #[test]
    fn test_formatting_token_to_str() {
        assert_eq!(FormattingToken::MetaSep.to_str(), "<|meta_sep|>");
        assert_eq!(FormattingToken::MetaEnd.to_str(), "<|meta_end|>");
    }

    #[test]
    fn test_harmony_encoding() {
        let encoding = HarmonyEncoding::new();
        assert_eq!(encoding.get_token_string(&FormattingToken::MetaSep), "<|meta_sep|>");
        assert_eq!(encoding.get_token_string(&FormattingToken::MetaEnd), "<|meta_end|>");
    }
}
