extern crate pest;
#[macro_use]
extern crate pest_derive; 

use pest::Parser;

#[derive(Parser)]
#[grammar = "parser.pest"]
pub struct AvantParser;

const CODE:&str="code;";

fn main() {
    println!("Hello, world!");
    let parse = AvantParser::parse(Rule::document, CODE)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails

    println!("{:?}", parse);
}
