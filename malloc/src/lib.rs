// Treat ourselves to a kb (1024 bits)
// 1024 >> 3 == 128 == 0x80
pub const SIZE: usize = 0x80;

// Not really a BUS but we gotta call it something.
static mut BUS: [u8; SIZE] = [0u8; SIZE];

const RESERVED_BYTES: usize = SIZE >> 3;

// Return an index in BUS of s reserved bytes
pub fn malloc(s: usize) -> Option<usize> {
    unsafe {
        if BUS[0] << 7 == 0 {
            init();
        }

        let mut byte = 0;
        let mut contig_size = 0;
        let mut contig_region_start = None;
        let mut bus_copy = BUS;
        for i in 0..RESERVED_BYTES {
            for bit in 0..=7 {
                if (BUS[i] >> bit) & 1 == 0 {
                    if contig_region_start.is_none() {
                        contig_region_start = Some(byte);
                        bus_copy = BUS;
                    }
                    bus_copy[i] |= 1 << bit;
                    contig_size += 1;
                    if contig_size == s {
                        BUS = bus_copy;
                        println!("malloc: returning address {}", contig_region_start.unwrap());
                        return contig_region_start;
                    }
                } else {
                    contig_size = 0;
                    contig_region_start = None;
                }
                byte += 1;
            }
        }
        print_bus();
    }
    println!("malloc: returning none");
    return None;
}

// Zero the array except the mask.
fn init() {
    unsafe {
        assert!(SIZE & (SIZE - 1) == 0);
        // First SIZE >> 3 bits are reserved as a validty byte/bit mask
        for i in 0..RESERVED_BYTES >> 3 {
            BUS[i] = !0;
        }
        println!("init:");
        print_bus()
    }
    return;
}

pub fn setter(value: i32, address: usize) {
    unsafe {
        let bytes: [u8; 4] = i32::to_ne_bytes(value);
        BUS[address] = bytes[0];
        BUS[address + 1] = bytes[1];
        BUS[address + 2] = bytes[2];
        BUS[address + 3] = bytes[3];
        println!("set {value:032b} starting at address {address}");
        print_bus();
    }
}

pub fn getter(address: usize) -> i32 {
    unsafe {
        let bytes: [u8; 4] = BUS[address..address + 4].try_into().unwrap();
        let value = i32::from_ne_bytes(bytes);
        value
    }
}

#[allow(static_mut_refs)]
fn print_bus() {
    println!();
    unsafe {
        for (i, byte) in BUS.iter().enumerate() {
            if i % 8 == 0 && i != 0 {
                println!();
            }
            print!("{:08b} ", byte);
        }
    }
    println!();
    println!();
}
