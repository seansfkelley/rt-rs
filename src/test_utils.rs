macro_rules! assert_close_to {
    ( $ left : expr , $ right : expr ) => {
        assert!(($left - $right).magnitude() < 1e-10);
    };
    ( $ left : expr , $ right : expr , $ ( $ arg : tt ) + ) => {
        assert!(($left - $right).magnitude() < 1e-10, $arg);
    };
}
