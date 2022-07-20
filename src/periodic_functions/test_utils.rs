macro_rules! test_panic {
    ($($value:path, $param:ident, $func:expr)*) => {
        $(
            paste!{
                #[test]
                #[should_panic]
                fn [<panics_on_ $value _ $param>]() {
                    let [<_ $value _ $param>] = $func;
                }
            }
        )*
    };
}

pub(crate) use test_panic;