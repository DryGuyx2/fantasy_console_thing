pub struct System {
    active_program: dyn Program,
}

impl System {
    pub fn new() -> Self {
        Self {}
    }
    pub fn load_program(&mut self, program: dyn Program) {
        self.active_program = program;
    }
}

trait Program {
    pub fn init();
    pub fn update();
}
