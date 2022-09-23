# bitstr

bitstr is just that: a sequence of bytes. It isn't human-readable. Under the hood, everything must be converted to a byte string before it can be stored in a computer.

Which means that a bitstring is a contiguous sequence of bits in memory.

```rust
// 1.
let bytes = [82, 105, 111];
let bit: &BitStr = BitStr::from(&bytes);

// 2.
let bytes = b"Rust";
let bit: &BitStr = BitStr::from(bytes);

// 3.
let bit: &BitStr = BitStr::from(b"My sequence of bytes");
let op = bit.contains([82, 105, 111]); // false

// 4.
let bit: &BitStr = BitStr::from(b"Rio");
let op = bit.contains_u8(82); // true
```

## TODO

- [ ] encode_utf16
- [ ] replace
- [ ] starts_with, ends_with
- [ ] get, get_mut
- [ ] get_unchecked, get_unchecked_mut
- [ ] slice_unchecked, slice_unchecked_mut