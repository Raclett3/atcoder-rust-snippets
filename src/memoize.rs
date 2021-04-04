#![allow(clippy::needless_range_loop)]

use cargo_snippet::snippet;

#[snippet("memoize")]
macro_rules! memoized {
    (fn $fn_name:ident ($($param:ident: $param_type:ty),+ $(,)?) -> $return_type:ty {$($body:tt)+}) => {
        fn $fn_name($($param: $param_type),+) -> $return_type {
            use std::collections::BTreeMap;
            use std::sync::Mutex;
            use lazy_static::lazy_static;

            fn body($($param: $param_type),+) -> $return_type {
                $($body)+
            }

            type ParamTypes = ($($param_type,)+);
            let params = ($($param,)+);
            lazy_static! {
                static ref MEMO: Mutex<BTreeMap<ParamTypes, $return_type>> = Mutex::new(BTreeMap::new());
            }

            if let Some(memoed_ret) = MEMO.lock().unwrap().get(&params) {
                return *memoed_ret;
            }

            let ret = body($($param),+);
            MEMO.lock().unwrap().insert(params, ret);
            ret
        }
    };
}

#[test]
fn test_memoize() {
    memoized! {fn fibonacci(n: usize) -> usize {
        match n {
            0 => 0,
            1 => 1,
            n => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }}

    assert_eq!(fibonacci(40), 102334155);
}
