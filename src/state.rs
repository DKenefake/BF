#[derive(Debug, Copy, Clone)]
pub struct ProgramState {
    pub ip: usize,
    pub dp: usize,
    pub memory: [i32; 30000],
}

impl ProgramState {
    pub fn new() -> Self {
        Self {
            ip: 0,
            dp: 0,
            memory: [0i32; 30000],
        }
    }

    pub fn is_valid_dp_location(&self, pos: usize) -> bool {
        pos < 30000
    }
}

impl Default for ProgramState {
    fn default() -> Self {
        Self::new()
    }
}