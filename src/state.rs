
pub struct ProgramState {
    pub ip: usize,
    pub dp: usize,
    pub memory: [i32; 30000],
}

impl ProgramState {
    pub fn new() -> ProgramState {
        ProgramState {
            ip: 0,
            dp: 0,
            memory: [0i32; 30000],
        }
    }
}