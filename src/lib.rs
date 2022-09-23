use std::cmp::Ordering;
use std::convert::AsRef;
use std::fmt::{Debug, Display, Formatter};
use std::mem::transmute;
use std::str::{from_utf8, Utf8Error};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BitStr {
    inner: [u8],
}

impl BitStr {
    #[inline]
    pub fn from(bytes: &[u8]) -> &Self {
        unsafe { transmute::<&[u8], &Self>(bytes) }
    }

    #[allow(clippy::should_implement_trait)]
    #[inline]
    pub fn from_str(bytes: &str) -> &Self {
        Self::from(bytes.as_bytes())
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.inner
    }

    #[inline]
    pub fn from_mut(bytes: &mut [u8]) -> &mut Self {
        unsafe { transmute::<&mut [u8], &mut Self>(bytes) }
    }

    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.inner
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }

    #[inline]
    pub fn first(&self) -> Option<u8> {
        self.inner.first().copied()
    }

    #[inline]
    pub fn first_mut(&mut self) -> Option<&mut u8> {
        self.inner.first_mut()
    }

    #[inline]
    pub fn last(&self) -> Option<u8> {
        self.inner.last().copied()
    }

    #[inline]
    pub fn last_mut(&mut self) -> Option<&mut u8> {
        self.inner.last_mut()
    }

    #[inline]
    pub fn split_first(&self) -> Option<(u8, &BitStr)> {
        self.inner.split_first().map(|(&a, b)| (a, BitStr::from(b)))
    }

    #[inline]
    pub fn split_last_mut(&mut self) -> Option<(&mut u8, &mut BitStr)> {
        self.inner
            .split_last_mut()
            .map(|(a, b)| (a, BitStr::from_mut(b)))
    }

    #[inline]
    pub fn split_at(&self, mid: usize) -> (&BitStr, &BitStr) {
        let (a, b) = self.inner.split_at(mid);
        (BitStr::from(a), BitStr::from(b))
    }

    #[inline]
    pub fn split_at_mut(&mut self, mid: usize) -> (&mut BitStr, &mut BitStr) {
        let (a, b) = self.inner.split_at_mut(mid);
        (BitStr::from_mut(a), BitStr::from_mut(b))
    }

    #[inline]
    pub fn contains_u8(&self, x: u8) -> bool {
        self.inner.contains(&x)
    }

    #[inline]
    pub fn contains(&self, x: u8) -> bool {
        self.inner.contains(&x)
    }

    #[inline]
    pub fn starts_with<T: AsRef<BitStr>>(&self, x: T) -> bool {
        self.inner.starts_with(x.as_ref().as_bytes())
    }

    #[inline]
    pub fn ends_with<T: AsRef<BitStr>>(&self, x: T) -> bool {
        self.inner.ends_with(x.as_ref().as_bytes())
    }

    #[inline]
    pub fn bytes(&self) -> std::iter::Cloned<std::slice::Iter<u8>> {
        self.inner.iter().cloned()
    }

    #[inline]
    pub fn bytes_mut(&mut self) -> std::slice::IterMut<u8> {
        self.inner.iter_mut()
    }

    #[inline]
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        from_utf8(self.as_bytes())
    }

    #[inline]
    pub fn is_ascii(&self) -> bool {
        self.inner.is_ascii()
    }

    #[inline]
    pub fn eq_ignore_ascii_case(&self, other: &BitStr) -> bool {
        self.inner.eq_ignore_ascii_case(&other.inner)
    }

    #[inline]
    pub fn make_ascii_uppercase(&mut self) {
        self.inner.make_ascii_uppercase()
    }

    #[inline]
    pub fn make_ascii_lowercase(&mut self) {
        self.inner.make_ascii_lowercase()
    }
}

impl AsRef<BitStr> for BitStr {
    #[inline]
    fn as_ref(&self) -> &BitStr {
        self
    }
}

impl AsRef<BitStr> for [u8] {
    #[inline]
    fn as_ref(&self) -> &BitStr {
        BitStr::from(self)
    }
}

impl AsRef<BitStr> for str {
    #[inline]
    fn as_ref(&self) -> &BitStr {
        BitStr::from(self.as_bytes())
    }
}

impl AsRef<[u8]> for BitStr {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}

impl<'a> Default for &'a BitStr {
    #[inline]
    fn default() -> Self {
        BitStr::from(&[])
    }
}

impl<'a> Default for &'a mut BitStr {
    #[inline]
    fn default() -> Self {
        BitStr::from_mut(&mut [])
    }
}

impl<'a> IntoIterator for &'a BitStr {
    type Item = u8;
    type IntoIter = std::iter::Cloned<std::slice::Iter<'a, u8>>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.bytes()
    }
}

impl<'a> IntoIterator for &'a mut BitStr {
    type Item = &'a mut u8;
    type IntoIter = std::slice::IterMut<'a, u8>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.bytes_mut()
    }
}

impl<'a> From<&'a [u8]> for &'a BitStr {
    #[inline]
    fn from(src: &'a [u8]) -> &'a BitStr {
        BitStr::from(src)
    }
}

impl Display for BitStr {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let s = String::from_utf8_lossy(self.inner.as_ref()).into_owned();
        f.write_str(&s);
        Ok(())
    }
}

impl Debug for BitStr {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let s = String::from_utf8_lossy(self.inner.as_ref()).into_owned();
        f.write_str(&s)
    }
}

macro_rules! impl_ord {
    ($t:ty) => {
        impl PartialEq<$t> for BitStr {
            #[inline]
            fn eq(&self, other: &$t) -> bool {
                <BitStr as PartialEq>::eq(self, other.as_ref())
            }
        }
        impl PartialEq<BitStr> for $t {
            #[inline]
            fn eq(&self, other: &BitStr) -> bool {
                <BitStr as PartialEq>::eq(self.as_ref(), other)
            }
        }
        impl PartialOrd<$t> for BitStr {
            #[inline]
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                <BitStr as PartialOrd>::partial_cmp(self, other.as_ref())
            }
        }
        impl PartialOrd<BitStr> for $t {
            #[inline]
            fn partial_cmp(&self, other: &BitStr) -> Option<Ordering> {
                <BitStr as PartialOrd>::partial_cmp(self.as_ref(), other)
            }
        }
    };
}

impl_ord!(str);
impl_ord!(&str);
impl_ord!([u8]);
impl_ord!(&[u8]);

#[test]
fn test_display() {
    let rio = BitStr::from(&[82, 105, 111]);
    assert_eq!(&format!("{}", rio), "Rio");
}

#[test]
fn test_debug() {
    let rio = BitStr::from(&[82, 105, 111]);
    assert_eq!(&format!("{:?}", rio), "Rio");
}

#[test]
fn test_contains_u8() {
    let str_bitstr: &BitStr = BitStr::from(&[82, 105, 111]);
    assert!(str_bitstr.contains_u8(82), "{}", true);
}

#[test]
fn test_first_and_last() {
    let bytes = [82, 105, 111];
    let bit_str: &BitStr = BitStr::from(&bytes);
    assert_eq!(111, bit_str.last().unwrap());
    assert_eq!(82, bit_str.first().unwrap());
}

#[test]
fn test_from() {
    let bytes = [82, 105, 111];
    let bit_str: &BitStr = BitStr::from(&bytes);

    let bt = b"Rust";
    let bit_str2: &BitStr = BitStr::from(bt);

    assert_eq!("Rio", bit_str);
    // assert_eq!(&[82, 105, 111], bit_str);
    assert_eq!("Rust", bit_str2);
    assert_eq!("Rust", bit_str2);
}
