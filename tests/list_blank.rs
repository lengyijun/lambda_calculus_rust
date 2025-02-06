#![cfg(feature = "encoding")]

extern crate lambda_calculus as lambda;

use lambda::data::turing_machine::list_blank::*;
use lambda::*;

macro_rules! test_list_blank {
    ($name:ident, $function:ident, $($($n:expr),+ => $result:expr),+) => (
        #[test]
        fn $name() {
            $(
                assert_eq!(
                    beta(app!($function(), $($n.into_church()),*), HAP, 0),
                    $result.into_church()
                );
            )*
        }
    );
}

fn nil() -> Vec<Term> {
    vec![]
} // a nil workaround for macro purposes

test_list_blank!(list_blank_head, head,
Vec::<usize>::new() => 0,
            vec![1] => 1,
         vec![1, 2] => 1,
      vec![1, 2, 3] => 1
);

test_list_blank!(list_blank_tail, tail,
 Vec::<usize>::new() =>         nil(),
             vec![1] =>         nil(),
          vec![1, 2] =>       vec![2],
       vec![1, 2, 3] =>    vec![2, 3],
    vec![1, 2, 3, 4] => vec![2, 3, 4]
);
