
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./ast/grammar.pest"] // Path to your grammar file
#[derive(Debug)]
struct ZoLangParser; // You can name your struct anything




pub fn parse_input(contents : &String){
           
    let pairs = ZoLangParser::parse(Rule::program, &contents).unwrap();

    for pair in pairs{
        println!("Text: {:#?}", pair.as_str());
        println!("Rule: {:#?}", pair.as_rule());
        println!("Span: {:#?}", pair.as_span());

    }

}