#![cfg(feature = "encoding")]

extern crate lambda_calculus;

use crate::data::boolean::fls;
use lambda_calculus::data::pair::pair;
use lambda_calculus::data::turing_machine::tape::*;
use lambda_calculus::*;

/// state → tape_head → <boolean, X>
///                         true, (write × move × state)
///                        false, state
fn halt_immediately() -> Term {
    abs!(2, app!(pair(), fls(), 1.into_church()))
}

#[test]
fn test_halt_immediately() {
    assert_eq!(beta(run(halt_immediately()), HSP, 0), 1.into_church());
    assert_eq!(beta(run(halt_immediately()), CBN, 0), 1.into_church());
    assert_eq!(beta(run(halt_immediately()), HNO, 0), 1.into_church());
    assert_eq!(beta(run(halt_immediately()), NOR, 0), 1.into_church());
}
