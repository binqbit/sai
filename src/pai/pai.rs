use std::{process::Command, thread};

use crate::{Message, ChatGPT, FUNCTIONS};



pub fn run_commands(commands: Vec<String>) {
    let thread_handle = thread::spawn(move || {
        for cmd in commands {
                println!("> {cmd}");
                let mut child = Command::new("cmd")
                    .arg("/C")
                    .arg(cmd)
                    .spawn()
                    .expect("Failed to execute command");

                let status = child.wait().expect("Failed to wait for command");

                if !status.success() {
                    eprintln!("status code error: {:?}", status);
                }
        }
    });
    thread_handle.join().expect("Failed to join thread");
}



pub fn print_text(texts: Vec<String>) {
    if texts.len() == 1 {
        println!("> print: {}", texts[0]);
    } else if texts.len() > 1 {
        println!("> print:");
        for text in texts {
            println!("{}", text);
        }
    }
}

pub fn read_file(name: String) -> String {
    println!("> read_file: {}", name);
    let contents = std::fs::read_to_string(name)
        .expect("Something went wrong reading the file");
    contents
}

pub fn write_file(name: String, content: String) {
    println!("> write_file: {}", name);
    std::fs::write(name, content)
        .expect("Something went wrong writing the file");
}

pub fn edit_text(gpt: &ChatGPT, text: String, description: String) -> Option<String> {
    println!("> edit_text: {}", description);
    let messages = vec![
        Message::new(String::from("system"), None, String::from("you need changes text according to user request and return the result in a format:\n```\ntext result\n```")),
        Message::new(String::from("user"), None, format!("{description}:\n{text}")),
    ];

    match gpt.send(messages, None) {
        Ok(Some(res)) => {
            if let (Some(start), Some(end)) = (res.find("```"), res.rfind("```")) {
                let res = res[start..end].to_string();
                if let Some(start) = res.find("\n") {
                    return Some(res[start..end].to_string());
                }
            }
        },
        Ok(None) => {
            println!("edit_text: None");
        },
        Err(err) => {
            println!("error: {}", err);
        },
    }

    None
}

pub fn list_dirs(path: String) -> Vec<String> {
    println!("> list_dirs: {}", path);
    if let Ok(paths) = std::fs::read_dir(std::env::current_dir().unwrap()) {
        let list = paths.collect::<Vec<_>>()
            .iter()
            .map(|path| path.as_ref().unwrap().path().display().to_string())
            .collect::<Vec<_>>();
        return list;
    } else {
        return vec![];
    }
}

pub fn pai_run(gpt: &ChatGPT, task: String, flags: Vec<String>) {
    let mut messages = vec![
        Message::new(String::from("system"), None, format!("you are an package manager assistant, your task is to help the user using only these functions: {}\ndon't invent your own functions!", FUNCTIONS.get_names().join(", "))),
        Message::new(String::from("system"), None, format!("user os info: {} {}", std::env::consts::OS, std::env::consts::ARCH)),
        Message::new(String::from("system"), None, format!("user current directory: {}", std::env::current_dir().unwrap().display())),
    ];

    if flags.contains(&String::from("-d")) {
        let list = list_dirs(std::env::current_dir().unwrap().to_str().unwrap().to_string())
            .join(", ");
        messages.push(Message::new(String::from("system"), None, format!("user dire~~ctories and files: {list}")));
    }

    messages.push(Message::new(String::from("user"), None, task));

    match gpt.send(messages, Some(FUNCTIONS.to_owned())) {
        Ok(Some(res)) => {
            println!("{}", res);
        },
        Err(err) => {
            println!("error: {}", err);
        },
        _ => {},
    }
}