pub trait BFExecuter {
    fn execute(&mut self);
    fn read_char(&mut self);
    fn write_char(&mut self);
    fn instruction_count(&mut self) -> usize;
    fn reset_machine_state(&mut self);
}
