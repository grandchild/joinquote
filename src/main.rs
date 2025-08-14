use argh::FromArgs;
use std::io::{self, BufRead, BufReader};

#[derive(FromArgs)]
/// join lines from stdin with configurable quoting
struct Args {
    /// separator (default: ", ")
    #[argh(positional, default = "String::from(\", \")")]
    separator: String,
    /// quote string, if "(", "[", "{", or "<" will be auto-paired
    #[argh(option, short = 'q', default = "String::from(\"\\\"\")")]
    quote: String,
    /// don't quote lines
    #[argh(switch, short = 'Q')]
    no_quotes: bool,
}

fn stdin_lines() -> Result<Vec<String>, io::Error> {
    BufReader::new(io::stdin().lock()).lines().collect()
}

fn get_quote_pair(quote: &str) -> (String, String) {
    match quote {
        "(" | ")" | "()" => ("(".to_string(), ")".to_string()),
        "[" | "]" | "[]" => ("[".to_string(), "]".to_string()),
        "{" | "}" | "{}" => ("{".to_string(), "}".to_string()),
        "<" | ">" | "<>" => ("<".to_string(), ">".to_string()),
        _ => (quote.to_string(), quote.to_string()),
    }
}

fn quote_and_join(lines: Vec<String>, args: &Args) -> String {
    lines
        .into_iter()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .map(|line| {
            if args.no_quotes {
                line
            } else {
                let (open_quote, close_quote) = get_quote_pair(&args.quote);
                format!("{}{}{}", open_quote, line, close_quote)
            }
        })
        .collect::<Vec<_>>()
        .join(&args.separator)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = argh::from_env();
    println!("{}", quote_and_join(stdin_lines()?, &args));
    Ok(())
}
