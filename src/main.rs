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

/// 差し替え用構造体
#[derive(Clone, Debug)]
pub struct ReplaceTarget {
    pub tag: String,
    pub target: String,

}

impl ReplaceTarget {
    pub fn new(target: String) -> Self {
        let mut input: String = "".to_string();
        let tag = format!("<{}>", target);
        // titleを標準入力から取得
        let stdin = stdin();
        print!("{}: ", target);
        stdout().flush().unwrap();
        stdin.lock().read_line(&mut input).unwrap();
        input = input.trim_end().to_owned().to_string();
        Self {
            tag: tag.to_string(),
            target: input,
        }
    }
}

/// テンプレートオブジェクト
#[derive(Clone, Debug)]
pub struct Template {
    pub template: String,
    pub input: Vec<ReplaceTarget>,
    pub replace_target_str: Vec<String>,
    pub result_template: String,
}

/// テンプレート実装
impl Template {
    pub fn new(temp_path: String) -> Self {
        let content = match fs::read_to_string(temp_path) {
            Ok(content) => content,
            Err(content) => {
                panic!("{}", content);
            }
        };

        Self {
            template: content,
            input: vec![],
            replace_target_str: vec![],
            result_template: "aaa".to_string()
        }
    }

    /// テンプレートを登録する
    pub fn create_replace_target(&mut self) -> &mut Self {
        for input in &self.input {
            self.replace_target_str.push(format!("s/{}/{}/g", input.tag, input.target).to_string());
        }
        self
    }

    /// テンプレートを差し替える
    pub fn exec_replace(&mut self) -> &mut Self {
        self.result_template = self.template.clone();
        for rep_str in &self.replace_target_str {
            self.result_template =  ReplaceCommand::new(&rep_str)
            .unwrap()
            .execute(self.result_template.as_str())
            .to_string();
        }
        self
    }
}

fn main() -> Result<(),Box<dyn std::error::Error>> {
    // template path
    let temp_path = "./README.tmpl.md";
    // init template object
    let mut template = Template::new(temp_path.to_string());
    // set template replace target
    template.input = vec![
        ReplaceTarget::new("title".to_string()),
        ReplaceTarget::new("design_man_hour".to_string())
    ];
    template.create_replace_target().exec_replace();
    println!("{}", template.result_template);
    Ok(())
}
