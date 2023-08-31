# rawbytes

A Rust crate to view a structure as a plain byte array (`&[u8]`).

Super simple. Tiny. Zero dependencies.

This is a safer interface to `slice::from_raw_parts_{mut}()`

Example usage:

```rust
use rawbytes::RawBytes;

#[repr(C, packed(4))])]
struct Foo {
    x: [u32; 32],
}

#[test]
fn test() {
    let mut foo = Foo { x: [0; 32] };

    let foo_bytes = RawBytes::bytes_view(&foo);
    assert_eq!(foo_bytes.len(), 128);

    let foo_bytes = RawBytes::bytes_view_mut(&mut foo);
    foo_bytes[0] = 1;
    assert_eq!(foo.x[0], 1);
}
```

Note that structures must should have the `C` representation to ensure that a new Rust release is not going to change the representation.

## Warning and alternatives

Warning: this crate contains two instances of the `unsafe` keyword, because there are no other ways to achieve this in Rust. Still, you may be named and shamed for using a crate that perfectly does the job, but includes that keyword.

An alternative is the `zerocopy` crate. It's bigger, far more complex, not any faster, and it also require the `unsafe` keyword. But it's maintained by a Google employee, so you may be less named and shamed.