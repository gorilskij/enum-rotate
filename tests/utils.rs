#[macro_export]
macro_rules! test_prev_next {
    ($prev:ident, $next:ident) => {
        ::paste::paste! {
            #[test]
            #[allow(non_snake_case)]
            fn [<test_prev_next_ $prev _ $next>]() {
                // TODO: turn into assert_matches! once that is stabilized
                assert!(matches!($prev.next(), $next), "{:?}.next() != {:?}", $prev, $next);
                assert!(matches!($next.prev(), $prev), "{:?}.prev() != {:?}", $next, $prev);

                let mut x = $prev;
                assert!(matches!(*x.rotate_next(), $next), "{:?}.rotate_next() != {:?}", $prev, $next);
                assert!(matches!(x, $next), "{:?}.rotate_next() doesn't result it {:?}", $prev, $next);

                let mut x = $next;
                assert!(matches!(*x.rotate_prev(), $prev), "{:?}.rotate_prev() != {:?}", $next, $prev);
                assert!(matches!(x, $prev), "{:?}.rotate_prev() doesn't result it {:?}", $next, $prev);
            }
        }
    };
}

#[macro_export]
macro_rules! test_iter_iter_from {
    ($enm:ident; [$( $x:ident ),*]; $( $v:ident: [$( $y:ident ),*]; )*) => {
        #[test]
        fn test_iter() {
            // Silence warning for the empty enum case
            #[allow(unused_mut)]
            let mut vec: Vec<_> = $enm::iter().collect();
            $(
                assert!(matches!(vec.remove(0), $x));
            )*
            assert!(vec.is_empty());
        }

        ::paste::paste! {
            $(
                #[test]
                #[allow(non_snake_case)]
                fn [<test_iter_from_ $v>]() {
                    let mut vec: Vec<_> = $enm::iter_from(&$v).collect();
                    $(
                        assert!(matches!(vec.remove(0), $y));
                    )*
                    assert!(vec.is_empty());
                }
            )*
        }
    };
}
