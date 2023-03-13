// The Chip-8 language is capable of accessing up to 4KB (4,096 bytes) of RAM, from location 0x000 (0) to 0xFFF (4095).
// The first 512 bytes, from 0x000 to 0x1FF, are where the original interpreter was located, and should not be used by
// programs.

// Most Chip-8 programs start at location 0x200 (512), but some begin at 0x600 (1536). Programs beginning at 0x600 
// are intended for the ETI 660 computer.

// Memory Map:
// +---------------+= 0xFFF (4095) End of Chip-8 RAM
// |               |
// |               |
// |               |
// |               |
// |               |
// | 0x200 to 0xFFF|
// |     Chip-8    |
// | Program / Data|
// |     Space     |
// |               |
// |               |
// |               |
// +- - - - - - - -+= 0x600 (1536) Start of ETI 660 Chip-8 programs
// |               |
// |               |
// |               |
// +---------------+= 0x200 (512) Start of most Chip-8 programs
// | 0x000 to 0x1FF|
// | Reserved for  |
// |  interpreter  |
// +---------------+= 0x000 (0) Start of Chip-8 RAM

struct Memory {
    storage: [u8],
}

impl Memory {
    fn new() -> Memory {
        Memory {storage: [0; 0xFFF]}
    }

    fn read_byte(index: u32) -> Option<byte> {
        if index > storage.len() {
            None
        }
        else {
            Some(storage[index])
        }
    }

    fn write_byte(word: byte, index: u32) {

    }
}