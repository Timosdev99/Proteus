use proteus::lexer::lexer::Lexer;
use proteus::lexer::token::Token;
use std::fs;

#[test]
fn lexes_sample_input() {
    let input = fs::read_to_string("tests/samples/input.ts")
        .expect("failed to read tests/samples/input.ts");
    let mut lexer = Lexer::new(&input);
    let tokens = lexer.tokenize();

    let expected = vec![
        Token::Function,
        Token::Identifier("addNumbers".to_string()),
        Token::LParen,
        Token::Identifier("a".to_string()),
        Token::Colon,
        Token::Identifier("number".to_string()),
        Token::Comma,
        Token::Identifier("b".to_string()),
        Token::Colon,
        Token::Identifier("number".to_string()),
        Token::RParen,
        Token::Colon,
        Token::Identifier("number".to_string()),
        Token::LBrace,
        Token::Return,
        Token::Identifier("a".to_string()),
        Token::Identifier("b".to_string()),
        Token::Semicolon,
        Token::RBrace,
        Token::EOF,
    ];

    assert_eq!(tokens, expected);
}
