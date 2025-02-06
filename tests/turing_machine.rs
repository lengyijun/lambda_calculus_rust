#![cfg(feature = "encoding")]

extern crate lambda_calculus as lambda;

use lambda::data::list_blank::*;
use lambda::*;

/// https://bbchallenge.org/1RB1LB_1LA1RZ
/// λ f state tape_head left right. (eq tape_head 0)
///                                     ((eq state 0)
///                                         (f 1 (head l) (tail l) (cons 1 r))
///                                         (f 1 (head r) (cons 1 l) (tail r))
///                                     )
///                                     ((eq state 0)
///                                         (f 0 (head r) (cons 1 l) (tail r))
///                                         1
///                                     )
fn bb2() -> Term() {
    app(Y, abs(term))
}
