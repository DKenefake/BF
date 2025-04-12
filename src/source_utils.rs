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

pub fn get_c_file_prefix() -> String{
    String::from(r#"#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

#define MEMSIZE 30000
#define IS_CHECK_ADDR

#ifdef _MSC_VER
#define _INLINE __forceinline
#elif __GNUC__
#define _INLINE __attribute__((always_inline)) inline
#else
#define _INLINE inline
#endif

_INLINE
void check_addr(uint32_t addr) {
	if (addr >= MEMSIZE) {
		printf("Tried to Access an illegal memory location at %d", addr);
		exit(1);
	}
}

_INLINE
void change(int32_t* m, uint32_t p, int32_t amount) {
	m[p] += amount;
}

_INLINE
uint32_t move(uint32_t p, int32_t amount) {

	#ifdef CHECK_ADDR
	check_addr(p + amount);
	#endif

	return p + amount;
}

_INLINE
void display_char(int32_t* m, uint32_t p) {
	putchar(m[p]);
}

_INLINE
int32_t read_char() {
	return getchar();
}

_INLINE
void set_to(int32_t* m, uint32_t p, int32_t amount) {
	m[p] = amount;
}

_INLINE
void multi(int32_t* m, uint32_t p, int32_t x, int32_t y) {
	int32_t pi = max((int32_t)p + x,0);
	m[pi] += y * m[p];
}

_INLINE
uint32_t scan_by(int32_t* m, uint32_t p, int32_t arg) {
	uint32_t new_p = p;

	while (m[new_p] != 0) {
		new_p = move(new_p, arg);
	}

	return new_p;
}

_INLINE
uint32_t moving_change(int32_t* m, uint32_t p, int32_t x, int32_t y, int32_t z) {

	uint32_t new_p = p;

	while (m[new_p] != 0) {
		new_p = move(new_p, x);
		change(m, new_p, y);
		new_p = move(new_p, z);
	}

	return new_p;
}

_INLINE
uint32_t moving_set(int32_t* m, uint32_t p, int32_t x, int32_t y, int32_t z) {

	uint32_t new_p = p;

	while (m[new_p] != 0) {
		new_p = move(new_p, x);
		set_to(m, new_p, y);
		new_p = move(new_p, z);
	}

	return new_p;
}

int main() {

	uint32_t p = 0;
	int32_t m[MEMSIZE] = { 0 };
	"#)
}
