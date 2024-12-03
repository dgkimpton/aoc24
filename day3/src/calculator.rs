#[derive(Debug, PartialEq)]
pub enum Operation {
    MUL(i32, i32),
    COND(bool),
}

#[derive(Debug)]
pub struct ElvishCalculator {
    enabled: bool,
    result: i32,
}

impl ElvishCalculator {
    pub fn new() -> Self {
        Self {
            result: 0,
            enabled: true,
        }
    }

    pub fn enter(&mut self, op: Operation) {
        match op {
            Operation::MUL(a1, a2) => {
                if self.enabled {
                    self.result += a1 * a2
                }
            }
            Operation::COND(toggle) => self.enabled = toggle,
        };
    }

    pub fn result(&self) -> i32 {
        self.result
    }
}
