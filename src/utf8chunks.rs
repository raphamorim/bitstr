use std::iter::FusedIterator;
use std::str::{from_utf8, from_utf8_unchecked};

pub struct Utf8ChunksIter<'a> {
    pub(super) bytes: &'a [u8],
}

pub struct Utf8Chunk<'a> {
    pub valid: &'a str,
    pub broken: &'a [u8],
}

impl<'a> Iterator for Utf8ChunksIter<'a> {
    type Item = Utf8Chunk<'a>;

    fn next(&mut self) -> Option<Utf8Chunk<'a>> {
        if self.bytes.is_empty() {
            return None;
        }
        match from_utf8(self.bytes) {
            Ok(s) => {
                self.bytes = &self.bytes[s.len()..];
                Some(Utf8Chunk {
                    valid: s,
                    broken: &self.bytes[..0],
                })
            }
            Err(e) => {
                let (valid, rest) = self.bytes.split_at(e.valid_up_to());
                let valid = unsafe { from_utf8_unchecked(valid) };
                let (broken, rest) = rest.split_at(e.error_len().unwrap_or(rest.len()));
                self.bytes = rest;
                Some(Utf8Chunk { valid, broken })
            }
        }
    }
}

impl<'a> FusedIterator for Utf8ChunksIter<'a> {}
