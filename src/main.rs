use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]

pub struct Grammar;


fn main() -> anyhow::Result<()> {
    
    let inputs = vec![
        "-273.15,-15\n",
        "0.00,100\n",
        "-1,42.42\n",
        "123.456,-654.321\n"
    ];

    for input in inputs {
    let got = Grammar::parse(Rule::file, &input)?;
    println!("{:?}", got);
    }

    Ok(())
}