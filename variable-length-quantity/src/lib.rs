#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut result = Vec::new();
    
    for &value in values.iter().rev() {
        let mut value = value as u64;
        result.push(value as u8 & 0x7F);
        
        value >>= 7;
        while value > 0 {
            result.push((value & 0x7F) as u8 | 0x80);
            value >>= 7;
        }
    }
    
    result.reverse();
    result
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    if !bytes.is_empty() && bytes[bytes.len() - 1] & 0x80 != 0 {
        return Err(Error::IncompleteNumber);
    }
    
    let mut result = Vec::new();
    let mut value = 0u64;

    for &byte in bytes {
        let byte = byte as u64;
        value = (value << 7) | (byte & 0x7F);
        if byte & 0x80 == 0 {
            result.push(value as u32);
            value = 0;
        } else if value & 0xFFFFFFFF00000000 != 0 {
            return Err(Error::IncompleteNumber);
        }
    }

    Ok(result)
}
