pub fn remove_no_coding_symbols(program_code: String) -> String {
    // removes anything that isn't a BF statement
    let valid_chars = ['+', '-', '<', '>', ',', '.', '[', ']'];
    let bf_valid_code = |c| valid_chars.contains(&c);

    let mut program_code = program_code;
    program_code.retain(bf_valid_code);

    program_code
}

pub fn check_program_brackets(program_code: &str) -> bool {
    // checks if the source has malformed brackets e.g. []][ or [[] or [[][][

    // if this ever goes negative then we know we have more closing than opening brackets
    // which is illegal
    let mut bracket_open_count = 0;

    for c in program_code.chars() {
        match c {
            '[' => {
                bracket_open_count += 1;
            }
            ']' => {
                if bracket_open_count == 0 {
                    return false;
                }
                bracket_open_count -= 1;
            }
            _ => {}
        }
    }

    // if we open and close the same number of brackets, this is valid bracket order
    bracket_open_count == 0
}
