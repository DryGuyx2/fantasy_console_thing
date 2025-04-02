use std::collections::HashMap;
use text_io::read;

pub struct Terminal {
    commands: HashMap<String, Command>,
}

type Command = fn(&mut Terminal, Vec<String>) -> String;

impl Terminal {
    pub fn new() -> Self {
        Self {
            commands: collect_commands(),
        }
    }
    pub fn start(&mut self) {
        loop {
            let input = UserInterface::input();
            let command = input.0;
            let args = input.1;
            if let Some(command) = self.commands.get(&command) {
                UserInterface::output(&command(&mut *self, args));
            } else {
                UserInterface::output(format!("Unexpected command: {}", &command).as_str());
            }
        }
    }
}

fn collect_commands() -> HashMap<String, Command> {
    let mut commands: HashMap<String, Command> = HashMap::new();
    commands.insert(String::from("echo"), |_, args| format!("{:?}", args));
    commands
}

struct UserInterface {}

impl UserInterface {
    pub fn output(text: &str) {
        println!("{}", text);
    }

    pub fn input() -> (String, Vec<String>) {
        print!(">");
        let raw_input: String = read!("{}\n");
        let mut parsed_input: Vec<String> =
            raw_input.split_whitespace().map(String::from).collect();
        (parsed_input.remove(0), parsed_input)
    }
}
