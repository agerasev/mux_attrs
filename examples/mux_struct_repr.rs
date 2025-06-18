use mux_attrs::{From, Mux};

#[derive(Mux)]
#[mux_names(b = B, c = C)]
#[derive(Clone, Copy, From, PartialEq, Eq, Debug)]
#[from(A, B, C)]
#[mux(b = repr(C),c = repr(C, packed))]
struct A(u8, u32);

fn main() {
    assert_eq!((size_of::<B>(), align_of::<B>()), (8, 4));
    assert_eq!((size_of::<C>(), align_of::<C>()), (5, 1));

    let a = A(123, 1234567890);
    assert_eq!(B::from(a), B(123, 1234567890));
    assert_eq!(C::from(a), C(123, 1234567890));
}
