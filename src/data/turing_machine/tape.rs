//! <https://leanprover-community.github.io/mathlib4_docs/Mathlib/Computability/Tape.html#Turing.Tape>
//! <https://github.com/lengyijun/goldbach_tm/blob/main/GoldbachTm/Tm25/TuringMachine25.lean>
//! tape = <tape_head, left, right>
//! cfg  = <state, tape>
//! stmt = <write, move>
//!
//! move:
//! left = move_left
//! right = move_right

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

/// λ tape b. (b, tape.2, tape.3)
pub fn write() -> Term {
    abs!(
        2,
        tuple!(Var(1), app(pi!(2, 3), Var(2)), app(pi!(3, 3), Var(2)))
    )
}

/// Tape -> Tape
/// λ <tape_head, left, right>. <head left, tail left, cons tape_head right>
pub fn move_right() -> Term {
    let tape_head = app(pi!(1, 3), Var(1));
    let left = app(pi!(2, 3), Var(1));
    let right = app(pi!(3, 3), Var(1));
    abs(tuple!(
        app(head(), left.clone()),
        app(tail(), left),
        app!(cons(), tape_head, right)
    ))
}

/// Tape -> Tape
/// λ <tape_head, left, right>. <head right, cons tape_head left, tail right>
pub fn move_left() -> Term {
    let tape_head = app(pi!(1, 3), Var(1));
    let left = app(pi!(2, 3), Var(1));
    let right = app(pi!(3, 3), Var(1));
    abs(tuple!(
        app(head(), right.clone()),
        app!(cons(), tape_head, left),
        app(tail(), right)
    ))
}

/// machine -> Cfg -> Option Cfg
/// machine -> Cfg -> <boolean, Cfg>
///                    true,    Cfg
///                    false,   state
fn step() -> Term {
    // o : <boolean, X>
    //         true, (write × move × state)
    //        false, state
    let o = app!(
        Var(2),             // machine
        app(fst(), Var(1)), // cfg.fst() = state
        app(pi!(1, 3), app(snd(), Var(1)))
    );
    let b = app(fst(), o.clone());

    app!(
        pair(),
        b.clone(),
        app!(
            b,
            abs(app!(
                pair(),
                app(pi!(3, 3), Var(1)),
                app(
                    app(pi!(2, 3), Var(1)), // move
                    app!(write(), app(snd(), Var(1)), app(pi!(2, 3), Var(1)))
                )
            )),
            I(),
            app(snd(), o.clone())
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
/// state → tape_head → Option (write × move × state)
/// state → tape_head → <boolean, X>
///                         true, (write × move × state)
///                        false, state
pub fn run(machine: impl Fn() -> Term) -> Term {
    let x = app!(step(), Var(2), Var(1));

    // machine -> Cfg -> state
    app!(
        Y(),
        app!(
            app(fst(), x.clone()),
            app!(Var(4), Var(3), Var(2)),
            I(),
            app(snd(), x)
        ),
        machine(),
        new_tape()
    )
}
