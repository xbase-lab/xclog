macro_rules! define
{ ($({
    ident: $name:ident,
    desc: $desc:literal,
    captures: [ $( $capture:ident ),* ],
    format: $format:literal,
    pattern: $pattern:literal,
    tests: { $($test_value:literal => $expr:expr),* }
     }),* $(,)?)
=>
{ paste::paste! {
    use lazy_static::lazy_static;
    use regex::{Regex, Captures as RegexCaptures};

    lazy_static! {
        /// Main Matcher for `PARSERS`
        pub static ref MATCHER: Matcher = Matcher::new();
    }

    $(
        #[doc = $name "Captures" "created by `" $name "Parser`" ]
        pub struct [<$name Match>]<'a> { _inner: RegexCaptures<'a> }
        impl<'a> [<$name Match>]<'a> {
            /// Pretty format
            pub fn format(&self) -> Option<String> {
                if $format.is_empty() { return None }
                $(
                    #[allow(unused_variables)]
                    let $capture = &self._inner[stringify!($capture)];
                 )*

                Some(format!($format))
            }

            #[doc = "Get data struct representation of `" $name "`"]
            pub fn data(&self, ) -> [<$name Data>] {
                [<$name Data>] { $($capture: self._inner[stringify!($capture)].to_string()),* }
            }
        }

        #[doc = "Static parser for " $name]
        pub struct [<$name Parser>] { re: Regex }
        impl [<$name Parser>] {
        #[doc = "Create enw instance of " $name " parser"]
            pub fn new(re: Regex) -> Self { Self { re } }

            /// Get captures from a text
            pub fn captures<'a>(&'a self, text: &'a str) -> Option<[<$name Match>]<'a>> {
                let captures = self.re.captures(text);

                if let Some(captures) = captures {
                    Some([<$name Match>] { _inner: captures })
                } else {
                    None
                }
            }
        }

        #[doc = "Data representation of " $name]
        pub struct [<$name Data>] { $(#[doc = $capture:upper] pub $capture: String),* }
    )*

    /// A enum with all possible matches
    pub enum Match<'a> { $(#[doc = $name " Match "] $name([<$name Match>]<'a>)),* }
    impl<'a> Match<'a> {
        /// Format capture as text
        pub fn format(&'a self) -> Option<String> {
            match self {
                $(Self::$name(v) => v.format(),)*

            }
        }
    }

    /// Collection of all supported parsers
    pub enum Parser { $(#[doc = "..."] $name([<$name Parser>])),* }
    impl Parser {
        pub fn capture<'a>(&'a self, text: &'a str) -> Option<Match<'a>> {
            match self {
                $(Self::$name(v) => v.captures(text).map(Match::$name),)*
            }
        }
    }


    /// Matchers Using a vector of [`Parser`]
    pub struct Matcher { inner: Vec<Parser> }
    impl Matcher {
        /// create new match instance
        pub fn new() -> Self {
            Self {
                inner: vec![
                    $(Parser::$name([<$name Parser>]::new(Regex::new($pattern).unwrap() ))),*
                ]
            }
        }

        /// Return [`Match`] if any thing is matched
        pub fn capture<'a>(&'a self, text: &'a str) -> Option<Match<'a>> {
            for parser in self.inner.iter() {
                let captures = parser.capture(text);
                if captures.is_some() {
                    return captures
                }
            }
            None
        }
    }

    // Tests ------------------------------------------------------------------------------------------------
    #[cfg(test)]
    mod tests {
        use regex::{Captures, Regex};
        use lazy_static::lazy_static;
        fn run_tests(captures: Captures, testfn: impl FnOnce(Captures)) {
            testfn(captures)
        }

        $(
            lazy_static! { static ref [<$name:snake:upper>]: Regex = Regex::new($pattern).unwrap(); }

            #[test]
            fn [<test_ $name:snake:lower>]() {
                $(
                    let captures = match [<$name:snake:upper>].captures($test_value) {
                        Some(cp)=> cp,
                        None => {
                            panic!("\nNo capture groups in\n\n```\n{}\n```\n\npattern:\n\n```\n{}\n```\n\n", $test_value, $pattern);
                        }
                    };
                    run_tests(captures, $expr);
                 )*
            }

         )*
    }
// --------------------------------------------------------------------------------------------------------
}};}

pub(crate) use define;
