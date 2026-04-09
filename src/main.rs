use proteus::lexer::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new(
        "
// function
function multiply(value: number, factor: number = 2): number {
  return value * factor;
}",
    );
    let tokens = lexer.tokenize();
    println!("{:#?}", tokens);
}
