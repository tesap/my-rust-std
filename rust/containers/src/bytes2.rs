use crate::vector as my;
use crate::chunks::Chunks;
use std::fmt;
use std::mem;
use std::ptr;


// TODO Is it possible?
//#[derive(ConstParamTy, PartialEq, Eq)]
//enum Endianness {
//    Little,
//    Big
//}

// === Formats ===
// bytes: (&[u8]) [x01, xf3, x7d, x19]
//   This serve as a common ground for all conversions
// int: (i64) 3485392
// bin: (str) "01010101010100110"
// hex: (str) "3bed02d1d149e5336349707ce47d6c90"


fn u8_to_bin(n: &u8) -> Bin {
    format!("{:08b}", n)
}

type Byte = u8;

#[derive(Debug)]
pub struct Bytes<const BIG_ENDIAN: bool = true>{
    pub vec: my::Vector<Byte>
}

#[derive(Clone)]
pub struct Hex(String);

type Bin = String;

#[derive(Debug)]
pub struct Bins(my::Vector<Bin>);

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
        println!();
    }
}

impl DebugBytes for i128 {
    fn print(&self) {
        let p: *const i128 = &*self;

        let view: Chunks<Byte> = Chunks {
            ptr: p as *mut Byte,
            count: 16
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

// TODO Complete trait
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

        let ptr: *const i128 = self.vec.as_ptr() as *const i128;
        unsafe {
            Ok(*ptr)
        }
    }

    pub fn to_bin(&self) -> Bins {
        let v: my::Vector<Bin> = self.vec.iter().map(u8_to_bin).collect();
        let a = Bins(v);
        a
    }

    //fn to_hex(&self) -> &str {
    //
    //}

    pub fn from_bytes(from: &[Byte]) -> Self {
        Self {
            vec: my::Vector::from_slice(from)
        }
    }

    // WHAT? If we pass 'from' by value, further from_slice fails with error referencing
    // i.e. value is dropped (right?)
    pub fn from_int(from: &i128) -> Self {
        // Directo cast doesn't work, but intermediate does
        let ptr: *const i128 = from;
        // Casting from const to mut is legal
        let ptr_u8: *mut u8 = ptr as *mut u8;

        // Why *mut type is needed?
        let nn_ptr = ptr::NonNull::new(ptr_u8).unwrap();
        let size: usize = mem::size_of::<i128>() / mem::size_of::<Byte>();

        // --> Doesnt work
        // let slice: *const [Byte] = ptr as *const [Byte];

        // Constructing wide pointer
        let slice = unsafe {
            ptr::NonNull::slice_from_raw_parts(nn_ptr, size).as_mut()
        };

        Self {
            vec: my::Vector::from_slice(slice)
        }
    }

    //fn from_bin(&self, from: Bins) -> Self {
    //    for i in 0..from.0.len() {
    //    }
    //}

    //fn from_hex(&self, from: &str) {
    //
    //}
}
