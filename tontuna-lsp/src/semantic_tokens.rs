use lsp_types::SemanticToken;
use tontuna::TokenKind;

pub fn get_semantic_tokens(src: &str) -> Vec<SemanticToken> {
    let mut result = Vec::new();

    let mut last_token_pos = (0, 0);
    let mut current_pos = (0, 0);
    let mut last_token_start = 0;

    for token in tontuna::tokens(src) {
        let start_offset = token.span.start().source_pos();
        let end_offset = token.span.end().source_pos();
        for c in src[last_token_start..start_offset].chars() {
            if c == '\n' {
                current_pos.0 += 1;
                current_pos.1 = 0;
            } else {
                current_pos.1 += c.len_utf16() as u32;
            }
        }

        let length = src[start_offset..end_offset].encode_utf16().count() as u32;

        let delta_line = current_pos.0 - last_token_pos.0;
        result.push(SemanticToken {
            delta_line,
            delta_start: current_pos.1 - if delta_line == 0 { last_token_pos.1 } else { 0 },
            length,
            token_type: token_type(token.kind),
            token_modifiers_bitset: 0,
        });
        last_token_pos = current_pos;
        last_token_start = start_offset;
    }

    result
}

pub const DEFINED_TYPES: &[&str] = &[
    "keyword", // TokenKind::Keyword
    "variable", // TokenKind::Value
    "operator", // TokenKind::Operator
    "punctuation", // TokenKind::Punctuation
    "number", // TokenKind::Number
    "comment", // TokenKind::Comment
    "string", // TokenKind::String
];

fn token_type(t: TokenKind) -> u32 {
    match t {
        TokenKind::Keyword => 0,
        TokenKind::Value => 1,
        TokenKind::Operator => 2,
        TokenKind::Punctuation => 3,
        TokenKind::Number => 4,
        TokenKind::Comment => 5,
        TokenKind::String => 6,
    }
}
