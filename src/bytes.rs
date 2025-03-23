use crate::Vector;
use std::fmt;
use std::mem;

// === Formats ===
// bytes: (&[u8]) [x01, xf3, x7d, x19]
//   This serve as a common ground for all conversions
// int: (i64) 3485392
// bin: (str) "01010101010100110"
// hex: (str) "3bed02d1d149e5336349707ce47d6c90"


fn u8_to_bin(n: &u8) -> Bin {
    format!("{:08b}", n)
}

fn bin_to_u8(b: &Bin) -> u8 {
    u8::from_str_radix(b, 2).expect("Invalid binary string")
}

fn u8_to_hex(n: &u8) -> String {
    format!("{:02x}", n)
}

fn hex_to_u8(h: &str) -> u8 {
    u8::from_str_radix(h, 16).expect("Invalid hex string")
}

type Byte = u8;

#[derive(Debug)]
pub struct Bytes<const BIG_ENDIAN: bool = true>{
    // TODO Switch to Chunks<Byte>
    pub vec: Vector<Byte>
}

#[derive(Clone, Debug)]
pub struct Hex(pub String);

pub type Bin = String;

#[derive(Debug)]
pub struct Bins(pub Vector<Bin>);

impl Into<String> for &Bins {
    fn into(self) -> String {
        self.0.join(" ")
    }
}

pub trait DebugBytes {
    fn print(&self);
}

impl fmt::Display for Hex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:}", self.0)
    }
}

impl DebugBytes for Bytes<true> {
    fn print(&self) {
        println!("\n----> Bytes: {:?}", self);
        self.as_slice().print();
        self.to_int128().unwrap().print();
        self.to_bin().print();
        self.to_hex().print();
        println!();
    }
}

impl DebugBytes for i128 {
    fn print(&self) {
        let p: *const i128 = &*self;

        let view: Vector<Byte> = Vector {
            ptr: p as *mut Byte,
            cap: 16,
            len: 16,
        };
        println!("-> i128: {:?}; {:?}", self, view);
        std::mem::forget(view);
    }
}

impl DebugBytes for &[Byte] {
    fn print(&self) {
        println!("-> &[u8]: {:?}", self);
    }
}

impl DebugBytes for Hex {
    fn print(&self) {
        println!("-> Hex: {:?}", self.0);
    }
}

impl DebugBytes for Bins {
    fn print(&self) {
        let s: String = self.into();
        println!("-> Bins: [{:}]", s);
    }
}

impl<const BE: bool> Bytes<BE> {
    pub fn as_slice(&self) -> &[Byte] {
        self.vec.as_slice()
    }

    pub fn to_int128(&self) -> Result<i128, String> {
        let s1 = size_of::<i128>();
        let s2 = self.vec.len_bytes();

        if s2 > s1 {
            return Err("Bytes length is too big".to_string());
        }

        unsafe {
            let ptr: *const i128 = self.vec.as_ptr() as *const i128;
            Ok(ptr.read())
        }
    }

    pub fn to_bin(&self) -> Bins {
        let v: Vector<Bin> = self.vec.iter().map(u8_to_bin).collect();
        let a = Bins(v);
        a
    }

    pub fn to_hex(&self) -> Hex {
        let hex_string = self.vec.iter()
            .map(u8_to_hex)
            .collect::<String>();
        Hex(hex_string)
    }

    pub fn from_bytes(from: &[Byte]) -> Self {
        Self {
            vec: Vector::from_slice_copy(from)
        }
    }

    // WHAT? If we pass 'from' by value, further from_slice fails with error referencing
    // i.e. value is dropped (right?)
    pub fn from_int(from: &i128) -> Self {
        // Direct cast doesn't work, but intermediate does
        let ptr: *const i128 = from;
        // Casting from const to mut is legal
        let ptr_u8: *mut u8 = ptr as *mut u8;

        // Why *mut type is needed?
        let size: usize = mem::size_of::<i128>() / mem::size_of::<Byte>();

        // --> Doesnt work
        // let slice: *const [Byte] = ptr as *const [Byte];

        // --> Is it prefferred option over direct slice constructor?
        // Constructing wide pointer
        // let nn_ptr = ptr::NonNull::new(ptr_u8).unwrap();
        //let slice = unsafe {
        //    ptr::NonNull::slice_from_raw_parts(nn_ptr, size).as_mut()
        //};

        // TODO Chunks
        // --> Manually create slice
        let slice = unsafe {
            std::slice::from_raw_parts(ptr_u8, size)
        };

        Self {
            vec: Vector::from_slice_copy(slice)
        }
    }

    pub fn from_bins(from: &Bins) -> Self {
        let bytes: Vector<Byte> = from.0.iter().map(bin_to_u8).collect();
        Self {
            vec: bytes
        }
    }

    pub fn from_hex(from: &Hex) -> Self {
        let mut v: Vector<Byte> = Vector::new(0);
        for i in (0..from.0.len()).step_by(2) {
            // What is &x[..] expression?
            let b: u8 = hex_to_u8(&from.0[i..i+2]);
            v.push(b);
        }

        Self {
            vec: v
        }
    }
}

impl From<Vector<u8>> for Bytes {
    fn from(value: Vector<u8>) -> Self {
        Self {
            vec: value
        }
    }
}

impl Into<Vector<u8>> for Bytes {
    fn into(self) -> Vector<u8> {
        self.vec
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(value: Vec<u8>) -> Self {
        Self {
            vec: Vector::from(value)
        }
    }
}

impl Into<Vec<u8>> for Bytes {
    fn into(self) -> Vec<u8> {
        self.vec.into()
    }
}

impl Default for Bytes {
    fn default() -> Self {
        Self {
            vec: Vector::new(0),
        }
    }
}
