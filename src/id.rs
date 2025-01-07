use std::{
    fmt::{Display, Formatter, LowerHex, Result as FmtResult, UpperHex}, str::FromStr, sync::Mutex,
    ops::{AddAssign, Add}
};

fn increment(seed: IDValue) -> IDValue {
    seed + 1
}

fn xor_shift_64(seed: IDValue) -> IDValue {
    let mut x = seed;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}

fn linear_congruential(seed: IDValue) -> IDValue {
    let a = 1103515245u128 as IDValue;
    let c = 12345;
    let m = (2 as IDValue).pow(31);
    a.wrapping_mul(seed)
        .wrapping_add(c)
        .checked_rem(m)
        .unwrap_or(65537)
}

fn sha512_id(seed: IDValue) -> IDValue {
    let hash = super::sha512(seed.to_be_bytes().as_ref());
    u128::from_be_bytes(hash[0..16].try_into().unwrap()) as IDValue
}

/// A function that generates a new ID from the given seed.
pub type IDGenerator = fn(IDValue) -> IDValue;
/// A unique identifier.
pub type IDValue = u128;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ID {
    id: IDValue,
    generator: IDGenerator,
}

impl ID {
    pub const DEFAULT_GENERATOR: IDGenerator = Self::SHA512;
    pub const XOR_SHIFT_64: IDGenerator = xor_shift_64;
    pub const LINEAR_CONGRUENTIAL: IDGenerator = linear_congruential;
    pub const SHA512: IDGenerator = sha512_id;
    pub const INCREMENT: IDGenerator = increment;

    pub fn new(id: IDValue) -> Self {
        ID {
            id,
            generator: Self::DEFAULT_GENERATOR,
        }
    }

    pub fn random() -> Self {
        Self::new(rand::random())
    }

    pub fn with_seed(mut self, seed: IDValue) -> Self {
        self.id = seed;
        self
    }

    pub fn with_generator(mut self, generator: IDGenerator) -> Self {
        self.generator = generator;
        self
    }

    pub fn next_id(&self) -> ID {
        Self {
            id: (self.generator)(self.id),
            generator: self.generator,
        }
    }

    pub fn join(&self, other: &ID) -> ID {
        let mut bytes = [0; 32];
        bytes[..16].copy_from_slice(&self.id.to_be_bytes());
        bytes[16..].copy_from_slice(&other.id.to_be_bytes());
        let hash = super::sha512(&bytes);
        let id = u128::from_be_bytes(hash[0..16].try_into().unwrap());
        ID::new(id)
    }
}

impl FromStr for ID {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Read in the bytes and then SHA512 them
        let bytes = s.as_bytes();
        let hash = super::sha512(bytes);
        // Convert the first 16 bytes to a u128
        let id = u128::from_be_bytes(hash[0..16].try_into().unwrap());
        Ok(ID::new(id))
    }
}

impl From<&str> for ID {
    fn from(s: &str) -> Self {
        s.parse::<Self>().unwrap_or_default()
    }
}
impl From<&String> for ID {
    fn from(s: &String) -> Self {
        s.parse::<Self>().unwrap_or_default()
    }
}

impl From<String> for ID {
    fn from(s: String) -> Self {
        s.parse::<Self>().unwrap_or_default()
    }
}

impl Default for ID {
    fn default() -> Self {
        ID::random().with_generator(ID::DEFAULT_GENERATOR)
    }
}

impl Display for ID {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // Convert to bytes
        let bytes = self.id.to_be_bytes();
        // Convert to hex
        let hex = bytes
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        write!(f, "{}", hex)
    }
}

impl LowerHex for ID {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // Convert to bytes
        let bytes = self.id.to_be_bytes();
        // Convert to hex
        let hex = bytes
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        write!(f, "{}", hex)
    }
}

impl UpperHex for ID {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // Convert to bytes
        let bytes = self.id.to_be_bytes();
        // Convert to hex
        let hex = bytes
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<String>();
        write!(f, "{}", hex)
    }
}

impl Iterator for ID {
    type Item = ID;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_id())
    }
}

impl AddAssign<ID> for ID {
    fn add_assign(&mut self, rhs: ID) {
        *self = self.join(&rhs);
    }
}

impl Add<ID> for ID {
    type Output = ID;

    fn add(self, rhs: ID) -> Self::Output {
        self.join(&rhs)
    }
}

impl From<i32> for ID {
    fn from(id: i32) -> Self {
        ID::new(id as IDValue)
    }
}

impl From<i64> for ID {
    fn from(id: i64) -> Self {
        ID::new(id as IDValue)
    }
}

impl From<usize> for ID {
    fn from(id: usize) -> Self {
        ID::new(id as IDValue)
    }
}