use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum MemCopyError {
    DestTooSmall {
        dest_len: usize,
        src_len: usize,
    },
    NullPointer,
}

impl fmt::Display for MemCopyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MemCopyError::DestTooSmall { dest_len, src_len } =>
                write!(
                    f,
                    "Destination buffer too small. Dest len: {}, Src len: {}",
                    dest_len,
                    src_len
                ),
            MemCopyError::NullPointer => write!(f, "Null pointer encountered"),
        }
    }
}

impl Error for MemCopyError {}

pub struct MemCopy;

impl MemCopy {
    pub fn fast_copy(dest: &mut [u8], src: &[u8]) -> Result<(), MemCopyError> {
        if dest.len() < src.len() {
            return Err(MemCopyError::DestTooSmall {
                dest_len: dest.len(),
                src_len: src.len(),
            });
        }
        unsafe {
            std::ptr::copy_nonoverlapping(src.as_ptr(), dest.as_mut_ptr(), src.len());
        }
        Ok(())
    }

    pub fn safe_copy(dest: &mut [u8], src: &[u8]) -> Result<(), MemCopyError> {
        if dest.len() < src.len() {
            return Err(MemCopyError::DestTooSmall {
                dest_len: dest.len(),
                src_len: src.len(),
            });
        }
        dest[..src.len()].copy_from_slice(src);
        Ok(())
    }

    // Copy part of the content, copy as much as possible
    pub fn partial_copy(dest: &mut [u8], src: &[u8]) -> usize {
        let copy_len = dest.len().min(src.len());
        dest[..copy_len].copy_from_slice(&src[..copy_len]);
        copy_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_copy() {
        let src = [1, 2, 3, 4, 5];
        let mut dest = [0; 5];

        assert!(MemCopy::fast_copy(&mut dest, &src).is_ok());
        assert_eq!(dest, src);
    }

    #[test]
    fn test_safe_copy() {
        let src = [1, 2, 3, 4, 5];
        let mut dest = [0; 5];

        assert!(MemCopy::safe_copy(&mut dest, &src).is_ok());
        assert_eq!(dest, src);
    }

    #[test]
    fn test_partial_copy() {
        let src = [1, 2, 3, 4, 5];
        let mut dest = [0; 3];

        let copied = MemCopy::partial_copy(&mut dest, &src);
        assert_eq!(copied, 3);
        assert_eq!(dest, [1, 2, 3]);
    }

    #[test]
    fn test_dest_too_small() {
        let src = [1, 2, 3, 4, 5];
        let mut dest = [0; 3];

        let result = MemCopy::fast_copy(&mut dest, &src);
        assert!(result.is_err());
        if let Err(MemCopyError::DestTooSmall { dest_len, src_len }) = result {
            assert_eq!(dest_len, 3);
            assert_eq!(src_len, 5);
        } else {
            panic!("Expected DestTooSmall error");
        }
    }

    #[test]
    fn test_empty_src() {
        let src: [u8; 0] = [];
        let mut dest = [0; 5];

        assert!(MemCopy::fast_copy(&mut dest, &src).is_ok());
        assert_eq!(dest, [0; 5]);
    }

    #[test]
    fn test_empty_dest() {
        let src = [1, 2, 3];
        let mut dest: [u8; 0] = [];

        let result = MemCopy::fast_copy(&mut dest, &src);
        assert!(result.is_err());
    }
}
