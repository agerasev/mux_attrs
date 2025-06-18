# mux_attrs

Attribute multiplexing.

Here this means repeating original code multiple times but with different attributes.

## Examples

### Different default enum variants

```rust
use mux_attrs::{From, Mux};

#[derive(Mux)]
#[mux_names(b = B, c = C)]
#[derive(Clone, Copy, From, PartialEq, Eq, Debug)]
#[from(A, B, C)]
#[mux(derive(Default))]
enum A {
    #[mux(b = default)]
    X,
    #[mux(c = default)]
    Y,
}
```

In this example `B::default()` returns `B::X`, `C::default()` returns `C::Y`, while `A` hasn't default constructor at all.
Also all these enums can be converted to each other via `from` or `into`.

### Different struct binary representation

```rust
use mux_attrs::{From, Mux};

#[derive(Mux)]
#[mux_names(b = B, c = C)]
#[derive(Clone, Copy, From, PartialEq, Eq, Debug)]
#[from(A, B, C)]
#[mux(b = repr(C),c = repr(C, packed))]
struct A(u8, u32);

assert_eq!((size_of::<B>(), align_of::<B>()), (8, 4));
assert_eq!((size_of::<C>(), align_of::<C>()), (5, 1));
```

## Derive macros

### `Mux`

Derive macro that actually performs multiplexing.

It simply repeats the item declaration multiple times except:
+ Original `#[derive(Mux, ...)]` attribute, but not subsequent.
+ The name of original item - it is replaced by names from `mux_names`.
+ `#[mux_names(...)]` attribute.
+ `#[mux(...)]` attributes - they are replaced by their content depending on keys.

`#[mux_names(...)]` attribute specifies names of repeated item.

There are two forms:
+ `#[mux_names(TypeName1)]` for single repetition only,
+ `#[mux_names(key1 = TypeName1, key2 = TypeName2, ...)]` for multiple repetitions.

`#[mux(...)]` attributes specifies attributes to substitute in repeated item.
The general form is `#[mux(default_attribute, key1 = attribute_for_item1, key2 = attribute_for_item2, ...)]`,
where keys are taken from `mux_names` macro.
If there's no both `default_attribte` and current repetiton key, then the attribute is simply skipped.

### `From`

Helper derive macro that implements casting between different types with the same fields or variants.

Requires attribute of form `#[from(A, B, C)]`, where `A`, `B` and `C` are types should be converted from.
If one of the types is the same as the name of the item itself then it is ignored. 

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
