#![cfg(feature = "encoding")]

extern crate lambda_calculus;

use lambda_calculus::data::boolean::tru;
use lambda_calculus::data::list::church::nil;
use lambda_calculus::data::pair::fst;
use lambda_calculus::data::pair::pair;
use lambda_calculus::data::pair::snd;
use lambda_calculus::data::turing_machine::tape::*;
use lambda_calculus::*;

#[test]
fn jiqian() {
    let x = app!(pair(), 0.into_church(), 1.into_church());
    let a = app!(fst(), x.clone());
    let b = app!(snd(), x.clone());
    let y = app!(pair(), a, b);

    assert_eq!(beta(x, NOR, 0), beta(y, NOR, 0),);
}

#[test]
fn jiting() {
    // let x = tuple!(0.into_church(), 1.into_church(), 2.into_church());
    let x = tuple!(Var(5), Var(6), Var(7));
    let a = app!(pi!(1, 3), x.clone());
    let b = app!(pi!(2, 3), x.clone());
    let c = app!(pi!(3, 3), x.clone());
    let y = tuple!(a.clone(), b.clone(), c.clone());

    // assert_eq!(beta(x.clone(), NOR, 0), beta(y, NOR, 0),);

    assert_eq!(
        beta(x, NOR, 0),
        beta(
            app!(abs!(3, tuple!(Var(4), Var(3), Var(2))), a, b, c),
            // app!(abs!(3, tuple!(Var(1), Var(2), Var(3))), a, b, c),
            // app!(abs!(3, tuple!(Var(1), Var(3), Var(2))), a, b, c),
            // app!(abs!(3, tuple!(Var(2), Var(1), Var(3))), a, b, c),
            // app!(abs!(3, tuple!(Var(2), Var(3), Var(1))), a, b, c),
            // app!(abs!(3, tuple!(Var(3), Var(1), Var(2))), a, b, c),
            // app!(abs!(3, tuple!(Var(3), Var(2), Var(1))), a, b, c),
            NOR,
            0
        ),
    );
    // assert_eq!(beta(x, HSP, 0), beta(y, HSP, 0),);
}

/*
#[test]
fn luyao() {
    let x = tuple!(0.into_church(), tru(), tru());
    let a = app!(pi!(1, 3), x.clone());
    let b = app!(pi!(2, 3), x.clone());
    let c = app!(pi!(3, 3), x.clone());
    let y = tuple!(a, b, c);
    // println!("x {:?}",&x);
    // println!("y {:?}",&y);
    // println!("y {:?}",beta(y.clone(), NOR, 0));

    assert_eq!(beta(x.clone(), NOR, 0), beta(y, NOR, 0),);
    // assert_eq!(beta(x, HSP, 0), beta(y, HSP, 0),);

    let z = app!(write(), 0.into_church(), x.clone());
    assert_eq!(beta(x.clone(), NOR, 0), beta(z, NOR, 0),);
}
 */

#[test]
fn test_new_tape() {
    assert_eq!(beta(app!(pi!(1, 3), new_tape()), HSP, 0), 0.into_church());
    assert_eq!(beta(app!(pi!(2, 3), new_tape()), HSP, 0), nil());
    assert_eq!(beta(app!(pi!(3, 3), new_tape()), HSP, 0), nil());
}

#[test]
fn test_write() {
    let x = app!(write(), 0.into_church(), new_tape());
    assert_eq!(beta(app!(pi!(2, 3), x.clone()), HSP, 0), nil(),);
    assert_eq!(beta(app!(pi!(3, 3), x.clone()), HSP, 0), nil());
    assert_eq!(beta(app!(pi!(1, 3), x.clone()), HSP, 0), 0.into_church());
}

#[test]
fn test_move_left() {
    let x = app!(move_left(), new_tape());
    assert_eq!(beta(app!(pi!(1, 3), x.clone()), HSP, 0), 0.into_church());
    assert_ne!(beta(app!(pi!(2, 3), x.clone()), HSP, 0), nil(),);
    assert_eq!(beta(app!(pi!(3, 3), x.clone()), HSP, 0), nil());
}

#[test]
fn test_move_right() {
    let x = app!(move_right(), new_tape());
    assert_eq!(beta(app!(pi!(1, 3), x.clone()), HSP, 0), 0.into_church());
    assert_eq!(beta(app!(pi!(2, 3), x.clone()), HSP, 0), nil(),);
    assert_ne!(beta(app!(pi!(3, 3), x.clone()), HSP, 0), nil());
}

/*
assert_eq!(
    beta(app!(write(), 0.into_church(), new_tape()), HSP, 0),
    new_tape(),
);
assert_eq!(
    beta(app!(write(), 0.into_church(), new_tape()), CBN, 0),
    new_tape(),
);
 */
