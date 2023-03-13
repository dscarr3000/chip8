

struct CPU {
    general_registers: [u8],
    i: u16,
    pc: u16,
    sp: u8,
    stack: [u16],
}

impl CPU {
    fn new() -> CPU {
        CPU {
            general_registers: [0; 16],
            i: 0,
            pc: 0,
            sp: 0,
            stack: [0; 16],
        }
    }
}

enum Instructions {
    SYS(u16),                       // 0nnn - SYS addr
    CLS,                            // 00E0 - CLS
    RET,                            // 00EE - RET
    JP(JumpTarget),                 // 1nnn - JP addr
                                    // Bnnn - JP V0, addr
    CALL(u16),                      // 2nnn - CALL addr
    SE(SkipTarget),                 // 3xkk - SE Vx, byte
                                    // 5xy0 - SE Vx, Vy
    SNE(SkipTarget),                // 4xkk - SNE Vx, byte
                                    // 9xy0 - SNE Vx, Vy
    LD(LoadTarget),                 // 6xkk - LD Vx, byte
                                    // 8xy0 - LD Vx, Vy
                                    // Annn - LD I, addr
                                    // Fx07 - LD Vx, DT
                                    // Fx0A - LD Vx, K
                                    // Fx15 - LD DT, Vx
                                    // Fx18 - LD ST, Vx
                                    // Fx29 - LD F, Vx
                                    // Fx33 - LD B, Vx
                                    // Fx55 - LD [I], Vx
                                    // Fx65 - LD Vx, [I]
    ADD(AddTarget),                 // 7xkk - ADD Vx, byte
                                    // 8xy4 - ADD Vx, Vy
                                    // Fx1E - ADD I, Vx
    OR(Registers, Registers),       // 8xy1 - OR Vx, Vy
    AND(Registers, Registers),      // 8xy2 - AND Vx, Vy
    XOR(Registers, Registers),      // 8xy3 - XOR Vx, Vy
    SUB(Registers, Registers),      // 8xy5 - SUB Vx, Vy
    SHR(Registers, Registers),      // 8xy6 - SHR Vx {, Vy}
    SUBN(Registers, Registers),     // 8xy7 - SUBN Vx, Vy
    SHL(Registers, Registers),      // 8xyE - SHL Vx {, Vy}
    RND(Registers, u8),             // Cxkk - RND Vx, byte
    DRW(Registers, Registers, u8),  // Dxyn - DRW Vx, Vy, nibble
    SKP(Registers),                 // Ex9E - SKP Vx
    SKNP(Registers),                // ExA1 - SKNP Vx
}

impl Instructions {
    fn to_opcode(&self) -> u16 {
        match self {
            SYS(addr) => (addr << 1) >> 1, // 0nnn
            CLS => 0x00E0,
            RET => 0x00EE,
            JP(target) => match target {
                Regular(addr) => (), // 1nnn
                Add(addr) => (), // Bnnn
            },
        }
    }
}

enum JumpTarget {
    Regular(u16),
    Add(u16),
}

enum SkipTarget {
    Byte(u8),
    Register(Registers),
}

enum LoadTarget {
    RegisterByte(Registers, u8),
    RegisterRegister(Registers, Registers),
    IAddr(u16),
    RegisterDT(Registers),
    RegisterK(Registers),
    DTRegister(Registers),
    STRegister(Registers),
    FRegister(Registers),
    BRegister(Regsiters),
    IRegister(Registers),
    RegisterI(Registers),
}

enum AddTarget {
    RegisterByte(Registers, u8),
    RegisterRegister(Registers, Registers),
    IRegister(Registers),
}