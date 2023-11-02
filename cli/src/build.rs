use std::io;

use casm::parse::{File, SourceMap, TokenStream};
use clap::Parser;

#[derive(Parser)]
pub struct BuildArgs {
    /// The input file to use
    input: String,
}

pub fn build(args: BuildArgs) -> io::Result<()> {
    let input = File::read(args.input)?;

    let mut source_map = SourceMap::new();
    let input = source_map.insert(input);

    let file = &source_map[input];
    let tokens = TokenStream::lex(&file.contents, input).unwrap();
    println!("{:#?}", tokens);

    Ok(())
}
