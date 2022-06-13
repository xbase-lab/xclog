macro_rules! define
{ ($({
    ident: $name:ident,
    kind: $kind:ident,
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
    use super::{MatchOutput, OutputKind};

    lazy_static! {
        /// Main Matcher for `PARSERS`
        pub static ref MATCHER: Matcher = Matcher::new();
    }

    $(
        #[doc = $name "Captures" "created by `" $name "Parser`" ]
        pub struct [<$name Match>]<'a> {
            _inner: RegexCaptures<'a>,
            kind: OutputKind,
        }

        impl<'a> [<$name Match>]<'a> {
            /// Pretty format
            pub fn output(&self) -> MatchOutput {
                if $format.is_empty() {
                    return  MatchOutput {
                        value: None,
                        kind: self.kind.clone(),
                    }
                }
                $(
                    #[allow(unused_variables)]
                    let $capture = &self._inner[stringify!($capture)];
                 )*
                let leading = match self.kind {
                    OutputKind::Error => "[Error] ",
                    OutputKind::Warning => "[Warn] ",
                    _ => "",

                };

                MatchOutput {
                    value: Some(format!("{}{}", leading, format!($format))),
                    kind: self.kind.clone(),
                }
            }

            #[doc = "Get data struct representation of `" $name "`"]
            pub fn as_data(&self) -> [<$name Data>] {
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
                    Some([<$name Match>] { _inner: captures, kind: OutputKind::$kind })
                } else {
                    None
                }
            }
        }

        #[doc = "Data representation of " $name]
        #[derive(Debug)]
        pub struct [<$name Data>] { $(#[doc = $capture:upper] pub $capture: String),* }
    )*

    /// A enum with all possible matches
    pub enum Match<'a> { $(#[doc = $name " Match "] $name([<$name Match>]<'a>)),* }
    impl<'a> Match<'a> {
        /// Format capture as text
        pub fn output(&'a self) -> MatchOutput {
            match self { $(Self::$name(v) => v.output(),)* }
        }

        /// Check whether match is error
        pub fn is_error(&'a self) -> bool {
            match self { $(Self::$name(_) => OutputKind::$kind.is_error(),)* }
        }

        /// Check whether match is warning
        pub fn is_task(&'a self) -> bool {
            match self { $(Self::$name(_) => OutputKind::$kind.is_task(),)* }
        }

        /// Check whether match is result
        pub fn is_result(&'a self) -> bool {
            match self { $(Self::$name(_) => OutputKind::$kind.is_result(),)* }
        }

        /// Check whether match is test
        pub fn is_test(&'a self) -> bool {
            match self { $(Self::$name(_) => OutputKind::$kind.is_test(),)* }
        }

        $(
            #[doc = "Check whether Match is `" $name "Match`"]
            pub fn [<is_ $name:snake:lower>](&self) -> bool {
                matches!(self, Match::$name(_))
            }

            #[doc = "Return some if Match is `" $name "Match`"]
            pub fn [<as_ $name:snake:lower>](&self) -> Option<&[<$name Match>]<'a>> {
                if let Match::$name(m) = self { Some(m) } else { None }
            }

            #[doc = "Return `" $name "Data` if match is " $name]
            pub fn [<as_ $name:snake:lower _data>](&self) -> Option<[<$name Data>]> {
                if let Match::$name(m) = self { Some(m.as_data()) } else { None }
            }

        )*
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
