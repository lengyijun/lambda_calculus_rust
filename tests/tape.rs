#![cfg(feature = "encoding")]

extern crate lambda_calculus;

use lambda_calculus::data::list::church::nil;
use lambda_calculus::data::turing_machine::tape::*;
use lambda_calculus::*;

#[test]
fn test_mmove() {
    assert_eq!(beta(app!(pi!(1, 3), new_tape()), HSP, 0), 0.into_church());
    assert_eq!(beta(app!(pi!(2, 3), new_tape()), HSP, 0), nil());
    assert_eq!(beta(app!(pi!(3, 3), new_tape()), HSP, 0), nil());
    // let x = app!(move_left(), new_tape());
    let x = app!(write(), 0.into_church(), new_tape());
    assert_eq!(
        beta(app!(pi!(2, 3), x.clone()), HSP, 0),
        beta(app!(pi!(2, 3), new_tape()), HSP, 0),
    );
    /*
    assert_eq!(
        beta(app!(pi!(3, 3), x), HSP, 0),
        beta(app!(pi!(3, 3), new_tape()), HSP, 0),
    );
     */
    // assert_eq!(beta(app!(move_right(), new_tape()), HSP, 0), new_tape());
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
