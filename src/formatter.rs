use super::t;
use colored::Colorize;


pub fn format_to_markdown(my_struct: t::Root) {
    println!();
    println!("{0}",my_struct[0].word.green().bold().underline());
    println!("");
    for phonetic in &my_struct[0].phonetics {
        if let Some(s) = &phonetic.text {
           println!("{}", s.yellow());
        }
    }

    println!("---------------------------------------------------------------------------------");

    if let Some(ori) = &my_struct[0].origin{
        println!("origin: {}", ori.blue().underline());
    }
    for meaning in &my_struct[0].meanings {
        println!("{}", meaning.part_of_speech.truecolor(134, 134, 239).underline().bold());
        for def in &meaning.definitions {
            let mut tmp = String::from("def");
            print!("{}: ",tmp.bold());
            println!("{}", def.definition.truecolor(253, 210, 224));
            if let Some(ex) = &def.example {
                tmp = String::from("e.g.");
                print!("{}: ", tmp.bold());
                println!("{}", ex.blue());
            }
        println!();
        }
    }
}
