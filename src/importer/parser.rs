use std::str::FromStr;
use math::*;
use core::*;
use importer::scene_builder::SceneBuilder;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__SceneFile {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use math::*;
    use core::*;
    use importer::scene_builder::SceneBuilder;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22camera__position_22(&'input str),
        Term_22camera__up_22(&'input str),
        Term_22look__at_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        NtNumber(f64),
        NtPoint(Point),
        NtSceneFile(()),
        NtVec3(Vec3),
        Nt____SceneFile(()),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        3, 4, 5, 0,
        // State 1
        -7, -7, -7, -7,
        // State 2
        0, 0, 0, 8,
        // State 3
        0, 0, 0, 8,
        // State 4
        0, 0, 0, 8,
        // State 5
        0, 0, 0, 8,
        // State 6
        -3, -3, -3, -3,
        // State 7
        -1, -1, -1, -1,
        // State 8
        0, 0, 0, 8,
        // State 9
        -4, -4, -4, -4,
        // State 10
        -5, -5, -5, -5,
        // State 11
        0, 0, 0, 8,
        // State 12
        0, 0, 0, 8,
        // State 13
        -2, -2, -2, -2,
        // State 14
        -6, -6, -6, -6,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -7,
        0,
        0,
        0,
        0,
        -3,
        -1,
        0,
        -4,
        -5,
        0,
        0,
        -2,
        -6,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 2, 0, 0,
        // State 1
        0, 0, 0, 0, 0,
        // State 2
        6, 7, 0, 0, 0,
        // State 3
        9, 0, 0, 10, 0,
        // State 4
        6, 11, 0, 0, 0,
        // State 5
        12, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0,
        // State 8
        13, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0,
        // State 11
        14, 0, 0, 0, 0,
        // State 12
        15, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""camera_position""###,
            r###""camera_up""###,
            r###""look_at""###,
            r###"r#"[0-9]+"#"###,
        ];
        __ACTION[(__state * 4)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_SceneFile<
        'input,
    >(
        builder: &mut SceneBuilder,
        input: &'input str,
    ) -> Result<(), __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (1, _) if true => 0,
                (2, _) if true => 1,
                (3, _) if true => 2,
                (0, _) if true => 3,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 4 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22camera__position_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22camera__up_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22look__at_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(builder, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(builder, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        builder: &mut SceneBuilder,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<(),__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Number = r#"[0-9]+"# => ActionFn(6);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(builder, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNumber(__nt), __end));
                0
            }
            2 => {
                // Point = Number, Number, Number => ActionFn(4);
                let __sym2 = __pop_NtNumber(__symbols);
                let __sym1 = __pop_NtNumber(__symbols);
                let __sym0 = __pop_NtNumber(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action4::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPoint(__nt), __end));
                1
            }
            3 => {
                // SceneFile = "camera_position", Point => ActionFn(1);
                let __sym1 = __pop_NtPoint(__symbols);
                let __sym0 = __pop_Term_22camera__position_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action1::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtSceneFile(__nt), __end));
                2
            }
            4 => {
                // SceneFile = "camera_up", Vec3 => ActionFn(2);
                let __sym1 = __pop_NtVec3(__symbols);
                let __sym0 = __pop_Term_22camera__up_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action2::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtSceneFile(__nt), __end));
                2
            }
            5 => {
                // SceneFile = "look_at", Point => ActionFn(3);
                let __sym1 = __pop_NtPoint(__symbols);
                let __sym0 = __pop_Term_22look__at_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action3::<>(builder, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtSceneFile(__nt), __end));
                2
            }
            6 => {
                // Vec3 = Number, Number, Number => ActionFn(5);
                let __sym2 = __pop_NtNumber(__symbols);
                let __sym1 = __pop_NtNumber(__symbols);
                let __sym0 = __pop_NtNumber(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action5::<>(builder, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtVec3(__nt), __end));
                3
            }
            7 => {
                // __SceneFile = SceneFile => ActionFn(0);
                let __sym0 = __pop_NtSceneFile(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(builder, input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 5 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22camera__position_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22camera__position_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22camera__up_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22camera__up_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22look__at_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22look__at_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNumber<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, f64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNumber(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPoint<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Point, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPoint(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSceneFile<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSceneFile(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtVec3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec3, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtVec3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____SceneFile<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____SceneFile(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__SceneFile::parse_SceneFile;
mod __intern_token {
    #![allow(unused_imports)]
    use std::str::FromStr;
    use math::*;
    use core::*;
    use importer::scene_builder::SceneBuilder;
    extern crate lalrpop_util as __lalrpop_util;
    extern crate regex as __regex;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            let __strs: &[&str] = &[
                "^(?u:[0-9])+",
                "^(?u:camera_position)",
                "^(?u:camera_up)",
                "^(?u:look_at)",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:[0-9])+").unwrap(),
                __regex::Regex::new("^(?u:camera_position)").unwrap(),
                __regex::Regex::new("^(?u:camera_up)").unwrap(),
                __regex::Regex::new("^(?u:look_at)").unwrap(),
            ];
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: __regex_set,
                regex_vec: __regex_vec,
            }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                let __matches = self.regex_set.matches(__text);
                if !__matches.matched_any() {
                    Some(Err(__lalrpop_util::ParseError::InvalidToken {
                        location: __start_offset,
                    }))
                } else {
                    let mut __longest_match = 0;
                    let mut __index = 0;
                    for __i in 0 .. 4 {
                        if __matches.matched(__i) {
                            let __match = self.regex_vec[__i].find(__text).unwrap();
                            let __len = __match.end();
                            if __len >= __longest_match {
                                __longest_match = __len;
                                __index = __i;
                            }
                        }
                    }
                    let __result = &__text[..__longest_match];
                    let __remaining = &__text[__longest_match..];
                    let __end_offset = __start_offset + __longest_match;
                    self.text = __remaining;
                    self.consumed = __end_offset;
                    Some(Ok((__start_offset, (__index, __result), __end_offset)))
                }
            }
        }
    }
}

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    builder: &mut SceneBuilder,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    builder: &mut SceneBuilder,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, p, _): (usize, Point, usize),
) -> ()
{
    {
    builder.camera_position(p);
  }
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    builder: &mut SceneBuilder,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, Vec3, usize),
) -> ()
{
    {
    builder.camera_up(v);
  }
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    builder: &mut SceneBuilder,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, p, _): (usize, Point, usize),
) -> ()
{
    {
    builder.look_at(p);
  }
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    builder: &mut SceneBuilder,
    input: &'input str,
    (_, f1, _): (usize, f64, usize),
    (_, f2, _): (usize, f64, usize),
    (_, f3, _): (usize, f64, usize),
) -> Point
{
    Point { x: f1, y: f2, z: f3 }
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    builder: &mut SceneBuilder,
    input: &'input str,
    (_, f1, _): (usize, f64, usize),
    (_, f2, _): (usize, f64, usize),
    (_, f3, _): (usize, f64, usize),
) -> Vec3
{
    Vec3 { x: f1, y: f2, z: f3 }
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    builder: &mut SceneBuilder,
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> f64
{
    f64::from_str(s).unwrap()
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
