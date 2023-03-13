// Chip-8 has 16 general purpose 8-bit registers, usually referred to as Vx, where x is a hexadecimal digit 
// (0 through F). There is also a 16-bit register called I. This register is generally used to store memory addresses, 
// so only the lowest (rightmost) 12 bits are usually used.

// The VF register should not be used by any program, as it is used as a flag by some instructions. See section 3.0, 
// Instructions for details.

// Chip-8 also has two special purpose 8-bit registers, for the delay and sound timers. When these registers are 
// non-zero, they are automatically decremented at a rate of 60Hz. See the section 2.5, Timers & Sound, for more 
// information on these.

// There are also some "pseudo-registers" which are not accessable from Chip-8 programs. The program counter (PC) 
// should be 16-bit, and is used to store the currently executing address. The stack pointer (SP) can be 8-bit, 
// it is used to point to the topmost level of the stack.

// The stack is an array of 16 16-bit values, used to store the address that the interpreter shoud return to when 
// finished with a subroutine. Chip-8 allows for up to 16 levels of nested subroutines.

struct Regsiters {
    v0: u8,
    v1: u8,
    v2: u8,
    v3: u8,
    v4: u8,
    v5: u8,
    v6: u8,
    v7: u8,
    v8: u8,
    v9: u8,
    va: u8,
    vb: u8,
    vc: u8,
    vd: u8,
    ve: u8,
    vf: u8,
    i: u16,
    pc: u16,
    sp: u8,
    stack: [u16],
}