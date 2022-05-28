#[cfg(test)]
pub mod tests {
    use lazy_regex::Captures;
    pub fn run_tests(captures: Captures, testfn: impl FnOnce(Captures)) {
        testfn(captures)
    }
}

macro_rules! define_pattern {
    (ident: $name:ident,
     desc: $desc:literal,
     captures: [ $( $capture:ident ),* ],
     pattern: $pattern:literal,
     tests: { $($test_value:literal => $expr:expr),* }) => {
        paste::paste! {
            #[doc = "Captures " $desc "\n\n"]
            $(
                #[doc = "- $" $capture "\n"]
            )*
            pub static [<$name>]: Lazy<Regex> = lazy_regex!($pattern);

            #[test]
            fn [<test_ $name:lower>]() {
                $(
                    let captures = match [<$name>].captures($test_value) {
                        Some(cp)=> cp,
                        None => {
                            panic!("\nNo capture groups in\n\n```\n{}\n```\n\npattern:\n\n```\n{}\n```\n\n", $test_value, $pattern);
                        }
                    };
                    crate::parser::util::tests::run_tests(captures, $expr);
                )*
            }
        }
    };
    (ident: $name: ident,
     desc: $desc: literal,
     captures: [ $( $capture: ident),* ],
     pattern: $pattern: literal
    ) => {
        define_pattern! { ident: $name, desc: $desc, captures: [$($capture), *], pattern: $pattern, tests: {}  }
   }
}
pub(crate) use define_pattern;
