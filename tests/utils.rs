#[macro_export]
macro_rules! test_prev_next {
    ($prev:ident, $next:ident) => {
        ::paste::paste! {
            #[test]
            #[allow(non_snake_case)]
            fn [<test_prev_next_ $prev _ $next>]() {
                assert_eq!($prev.next(), $next, "{:?}.next() != {:?}", $prev, $next);
                assert_eq!($next.prev(), $prev, "{:?}.prev() != {:?}", $next, $prev);

                let mut x = $prev;
                assert_eq!(x.rotate_next(), $next, "{:?}.rotate_next() != {:?}", $prev, $next);
                assert_eq!(x, $next, "{:?}.rotate_next() doesn't result it {:?}", $prev, $next);

                let mut x = $next;
                assert_eq!(x.rotate_prev(), $prev, "{:?}.rotate_prev() != {:?}", $next, $prev);
                assert_eq!(x, $prev, "{:?}.rotate_prev() doesn't result it {:?}", $next, $prev);
            }
        }
    };
}
