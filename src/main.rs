use std::io::BufRead;
use sedregex::ReplaceCommand;
use std::{
    fs,
    io::{
        stdin,
        stdout,
        Write
    }
};

fn main() -> Result<(),Box<dyn std::error::Error>> {
    let template = "./README.tmpl.md";
    let mut title: String = "".to_string();
    let rep_target = "<title>".to_string();
    // titleを標準入力から取得
    let stdin = stdin();
    print!("title: ");
    stdout().flush().unwrap();
    stdin.lock().read_line(&mut title)?;
    title = title.trim_end().to_owned().to_string();

    let rep = set_replace_target(rep_target, title);
    // for result in BufReader::new(File::open(template)?).lines() {
    //     let l = result?;
    //     let rep = ReplaceCommand::new(&replace).unwrap().execute(l);
    //     println!("{}", rep);
    // }
    let content = match fs::read_to_string(template) {
        Ok(content) => content,
        Err(content) => {
            panic!("{}", content);
        }
    };
    println!("{}", replace_content(content, rep));
    Ok(())
}

fn set_replace_target(rep_target: String, rep_str: String) -> String {
    format!("s/{}/{}/g", rep_target, rep_str).to_string()
}

fn replace_content(content: String, replace_string: String) -> String {
    ReplaceCommand::new(&replace_string).unwrap().execute(content).to_string()
}
