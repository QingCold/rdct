mod t;
mod formatter;
use std::env;
use std::process;
use serde_json::from_str;  

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {
    let url = String::from("https://api.dictionaryapi.dev/api/v2/entries/en/");
    let args :Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: rdct <word>");
        process::exit(1);
    }
    let word = args[1].clone();

    let resp = reqwest::get(url+&word)
        .await?
        .text()
        .await?;
    let serialized: t::Root = match from_str(&resp) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("sorry, current word is unrecognisable !");
            process::exit(1); 
        }
    };

    formatter::format_to_markdown(serialized);
    Ok(())
}

