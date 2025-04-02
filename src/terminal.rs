use std::collections::HashMap;
use text_io::read;
use mlua::prelude::*;

pub struct Terminal {
  commands: HashMap<String, Command>,
  lua_vm: Lua,
}

type Command = fn(&mut Terminal) -> String;

impl Terminal {
  pub fn new() -> Self {
    Self {commands: collect_commands(), lua_vm: Lua::new()}
  }
  pub fn start(&mut self) {
    loop {
      let input_command = UserInterface::input();
      if let Some(command) = self.commands.get(&input_command) {
        UserInterface::output(&command(&mut *self));
      } else {
        UserInterface::output("Unexpected command.");
      }
    }
  }
}

fn collect_commands() -> HashMap<String, Command> {
  let mut commands: HashMap<String, Command> = HashMap::new();
  commands.insert(String::from("test"), |_| String::from("TEST"));
  commands.insert(String::from("eval"), eval);
  commands
}

fn eval(_: &mut Terminal) -> String {
  lua.load("").exec().unwrap();
}

struct UserInterface {}

impl UserInterface {
  pub fn output(text: &str) {
    println!("{}", text);
  }
  pub fn input() -> String {
    print!(">");
    read!()
  }
}
