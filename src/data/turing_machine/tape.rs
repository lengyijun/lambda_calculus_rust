//! <https://leanprover-community.github.io/mathlib4_docs/Mathlib/Computability/Tape.html#Turing.Tape>
//! <https://github.com/lengyijun/goldbach_tm/blob/main/GoldbachTm/Tm25/TuringMachine25.lean>
//! tape = <tape_head, left, right>
//!

use crate::combinators::{I, Y};
use crate::data::num::convert::IntoChurchNum;
use crate::data::pair::{fst, pair, snd};
use crate::data::turing_machine::list_blank::*;
use crate::term::Term::*;
use crate::term::{abs, app, Term};

/// <tape_head, left, right>
pub fn new_tape() -> Term {
    tuple!(0.into_church(), nil(), nil())
}

/// λ b tape. (b, tape.2, tape.3)
pub fn write() -> Term {
    abs!(
        2,
        tuple!(Var(3), app(pi!(2, 3), Var(2)), app(pi!(3, 3), Var(2)))
    )
}

/// Tape -> Tape
/// λ <tape_head, left, right>. <head left, tail left, cons tape_head right>
pub fn move_right() -> Term {
    let tape_head = app(pi!(1, 3), Var(2));
    let left = app(pi!(2, 3), Var(2));
    let right = app(pi!(3, 3), Var(2));
    abs(tuple!(
        app(head(), left.clone()),
        app(tail(), left),
        app!(cons(), tape_head, right)
    ))
}

/// Tape -> Tape
/// λ <tape_head, left, right>. <head right, cons tape_head left, tail right>
pub fn move_left() -> Term {
    let tape_head = app(pi!(1, 3), Var(2));
    let left = app(pi!(2, 3), Var(2));
    let right = app(pi!(3, 3), Var(2));
    abs(tuple!(
        app(head(), right.clone()),
        app!(cons(), tape_head, left),
        app(tail(), right)
    ))
}

/// machine -> (state × tape) -> <boolean, X>
///                                  true, (state × tape)
///                                 false, state
/// @parameter machine:
/// state → tape_head → <boolean, X>
///                         true, (write × move × state)
///                        false, state
pub fn step() -> Term {
    // o : <boolean, X>
    //         true, (write × move × state)
    //        false, state
    let o = app!(
        Var(2),             // machine
        app(fst(), Var(1)), // (state × tape).fst() = state
        app(pi!(1, 3), app(snd(), Var(1)))
    );
    let b = app(fst(), o.clone());

    abs!(
        2,
        app!(
            pair(),
            b.clone(),
            app!(
                b,
                // (write × move × state) -> (state × tape)
                abs(
                    // Var(1) : (write × move × state)
                    // Var(2) : (state × tape)
                    // Var(3) : machine
                    app!(
                        pair(),
                        app(pi!(3, 3), Var(1)),
                        app(
                            app(pi!(2, 3), Var(1)), // move
                            app!(write(), app(pi!(1, 3), Var(1)), app(snd(), Var(2)))
                        )
                    )
                ),
                I(),
                app(snd(), o.clone())
            )
        )
    )
}

/// It is suitable for `NOR` (normal), `HNO` (hybrid normal), `CBN` (call-by-name) and `HSP`
/// (head spine) reduction `Order`s.
///
/// @result:
/// If beta-reduction of result halts, then turing machine halts
/// If beta-reduction of result never halts, then turing machine never halts
///
/// @parameter machine:
/// state → tape_head → <boolean, X>
///                         true, (write × move × state)
///                        false, state
pub fn run(machine: impl Fn() -> Term) -> Term {
    //  <boolean, X>
    //      true, (state × tape)
    //     false, state
    let x = app!(step(), Var(2), Var(1));

    // machine -> (state × tape) -> state
    app!(
        Y(),
        abs!(
            3,
            app!(
                app(fst(), x.clone()),
                app!(Var(4), Var(3), Var(1)),
                I(),
                app(snd(), x)
            )
        ),
        machine(),
        app!(pair(), 0.into_church(), new_tape())
    )
}
