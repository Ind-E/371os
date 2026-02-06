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
                        print_bus();
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
    }
    print_bus();
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
}

pub fn setter<T>(value: T, address: usize) {
    unsafe {
        (&raw mut BUS)
            .cast::<u8>()
            .add(address)
            .cast::<T>()
            .write_unaligned(value);
    }
    println!("setting byte {address}");
    print_bus();
}

pub fn getter<T>(address: usize) -> T {
    unsafe {
        (&raw mut BUS)
            .cast::<u8>()
            .add(address)
            .cast::<T>()
            .read_unaligned()
    }
}

fn print_bus() {
    println!();
    let mut i = 0;
    unsafe {
        for byte in BUS {
            if i % 8 == 0 && i != 0 {
                println!();
            }
            print!("{:08b} ", byte);
            i += 1;
        }
    }
    println!();
    println!();
}
