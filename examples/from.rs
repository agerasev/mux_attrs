use mux_attrs::{From, Mux};

#[derive(Mux)]
#[mux_names(B)]
#[derive(From)]
#[from(A, B)]
struct A<T> {
    x: i32,
    y: T,
}

#[derive(Mux)]
#[mux_names(D)]
#[derive(From)]
#[from(C, D)]
enum C {
    U,
    T(i32, bool),
    S { x: i32, y: bool },
}

fn main() {
    assert!(matches!(
        A::<bool> { x: 123, y: true }.into(),
        B::<bool> { x: 123, y: true },
    ));
    assert!(matches!(
        B::<char> { x: 321, y: 'a' }.into(),
        A::<char> { x: 321, y: 'a' },
    ));

    assert!(matches!(C::U.into(), D::U));
    assert!(matches!(D::U.into(), C::U));
    assert!(matches!(C::T(123, true).into(), D::T(123, true)));
    assert!(matches!(D::T(321, false).into(), C::T(321, false)));
    assert!(matches!(
        C::S { x: 123, y: true }.into(),
        D::S { x: 123, y: true },
    ));
    assert!(matches!(
        D::S { x: 321, y: false }.into(),
        C::S { x: 321, y: false },
    ));
}
