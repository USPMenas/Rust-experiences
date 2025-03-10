# Parser

The following implements a very simple parser for an expression language. However, it handles errors by panicking.
Rewrite it to instead use idiomatic error handling and propagate errors to a return from main.

Feel free to use `thiserror` and `anyhow` (see https://crates.io/crates/thiserror and https://crates.io/crates/anyhow).

Start by fixing error handling in the `parse` function. Once that is working correctly, update `Tokenizer` to implement `Iterator<Item=Result<Token, TokenizerError>>` and handle that in the parser.

Feel free to add tests to ensure your parser raises the appropriate errors you will define.