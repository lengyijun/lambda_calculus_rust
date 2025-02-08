#![cfg(feature = "encoding")]

extern crate lambda_calculus;

use crate::data::boolean::{fls, tru};
use lambda_calculus::data::list::church::{cons, nil};
use lambda_calculus::data::num::church::eq;
use lambda_calculus::data::pair::pair;
use lambda_calculus::data::turing_machine::tape::*;
use lambda_calculus::*;

/// state → tape_head → <boolean, X>
///                         true, (write × move × state)
///                        false, state
///
/// https://bbchallenge.org/1RB1LB_1LA1RZ
fn bb2() -> Term {
    abs!(
        2,
        app!(
            eq(),
            Var(2),
            0.into_church(),
            // state = A
            app!(
                eq(),
                Var(1),
                0.into_church(),
                app!(
                    pair(),
                    tru(),
                    tuple!(1.into_church(), move_right(), 1.into_church())
                ),
                app!(
                    pair(),
                    tru(),
                    tuple!(1.into_church(), move_left(), 1.into_church())
                )
            ),
            // state = B
            app!(
                eq(),
                Var(1),
                0.into_church(),
                app!(
                    pair(),
                    tru(),
                    tuple!(1.into_church(), move_left(), 0.into_church())
                ),
                app!(pair(), fls(), 1.into_church())
            )
        )
    )
}

#[test]
fn test_machine() {
    assert_eq!(
        beta(app!(bb2(), 0.into_church(), 0.into_church()), HSP, 0),
        beta(
            app!(
                pair(),
                tru(),
                tuple!(1.into_church(), move_right(), 1.into_church())
            ),
            HSP,
            0
        )
    );

    assert_eq!(
        beta(app!(bb2(), 0.into_church(), 1.into_church()), HSP, 0),
        beta(
            app!(
                pair(),
                tru(),
                tuple!(1.into_church(), move_left(), 1.into_church())
            ),
            HSP,
            0
        )
    );

    assert_eq!(
        beta(app!(bb2(), 1.into_church(), 0.into_church()), HSP, 0),
        beta(
            app!(
                pair(),
                tru(),
                tuple!(1.into_church(), move_left(), 0.into_church())
            ),
            HSP,
            0
        )
    );

    assert_eq!(
        beta(app!(bb2(), 1.into_church(), 1.into_church()), HSP, 0),
        beta(app!(pair(), fls(), 1.into_church()), HSP, 0)
    );
}

#[test]
fn test_step() {
    assert_eq!(
        beta(
            app!(
                step(),
                bb2(),
                app!(
                    pair(),
                    1.into_church(),
                    tuple!(1.into_church(), nil(), nil())
                )
            ),
            NOR,
            0,
        ),
        beta(app!(pair(), fls(), 1.into_church()), NOR, 0)
    );

    // state B, new tape
    assert_eq!(
        beta(
            app!(step(), bb2(), app!(pair(), 1.into_church(), new_tape())),
            NOR,
            0,
        ),
        beta(
            app!(
                pair(),
                tru(),
                app!(
                    pair(),
                    0.into_church(), // state A
                    tuple!(0.into_church(), app!(cons(), 1.into_church(), nil()), nil())
                )
            ),
            NOR,
            0
        )
    );

    // state A, new tape
    assert_eq!(
        beta(
            app!(step(), bb2(), app!(pair(), 0.into_church(), new_tape())),
            NOR,
            0,
        ),
        beta(
            app!(
                pair(),
                tru(),
                app!(
                    pair(),
                    1.into_church(), // state B
                    tuple!(0.into_church(), nil(), app!(cons(), 1.into_church(), nil()))
                )
            ),
            NOR,
            0
        )
    );

    // state A,
    // tape = <nil> 1 <nil>
    assert_eq!(
        beta(
            app!(
                step(),
                bb2(),
                app!(
                    pair(),
                    0.into_church(),
                    tuple!(1.into_church(), nil(), nil())
                )
            ),
            NOR,
            0,
        ),
        beta(
            app!(
                pair(),
                tru(),
                app!(
                    pair(),
                    1.into_church(), // state B
                    tuple!(0.into_church(), app!(cons(), 1.into_church(), nil()), nil())
                )
            ),
            NOR,
            0
        )
    );
}

#[test]
fn test_bb2() {
    assert_eq!(beta(run(bb2), HSP, 0), 1.into_church()); // wrong answer

    // assert_eq!(beta(run(bb2), CBN, 0), 1.into_church()); // wrong answer
    // assert_eq!(beta(run(bb2), HNO, 0), 1.into_church()); // stack overflow
    // assert_eq!(beta(run(bb2), NOR, 0), 1.into_church()); // stack overflow
}
