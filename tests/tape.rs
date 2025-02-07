#![cfg(feature = "encoding")]

extern crate lambda_calculus;

use lambda_calculus::data::turing_machine::tape::*;
use lambda_calculus::*;

#[test]
fn test_mmove() {
    assert_eq!(
        beta(app!(write(), 0.into_church(), new_tape()), HSP, 0),
        new_tape(),
    );
    assert_eq!(
        beta(app!(write(), 0.into_church(), new_tape()), CBN, 0),
        new_tape(),
    );
    assert_eq!(beta(app!(move_left(), new_tape()), HSP, 0), new_tape());
    assert_eq!(beta(app!(move_right(), new_tape()), HSP, 0), new_tape());
}
