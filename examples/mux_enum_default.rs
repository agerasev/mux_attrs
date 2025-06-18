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

fn main() {
    assert_eq!(B::from(A::X), B::default());
    assert_eq!(C::from(A::Y), C::default());
}
