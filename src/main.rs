mod terminal;

use terminal::Terminal;

fn main() {
    let mut terminal = Terminal::new();
    terminal.start();
}
