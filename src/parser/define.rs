macro_rules! define
{ ($({
    ident: $name:ident,
    desc: $desc:literal,
    captures: [ $( $capture:ident ),* ],
    pattern: $pattern:literal,
    tests: { $($test_value:literal => $expr:expr),* }
     }),* $(,)?)
=>
{ paste::paste! {

    // use crate::parser::output::*;
    use lazy_static::lazy_static;
    use regex::{Regex, Captures};
    use regex::{RegexSet};

    /// All possible patterns that XCode Log might result to.
    // #[derive(PartialEq, Eq)]
    // pub enum MatchKind {
    //     $(
    //         #[doc = "`"$name "`: " $desc "\n\nCaptures"]
    //         $(#[doc = "- $" $capture "\n"])*
    //         $name
    //      ),*
    // }


    // Helpers ---------------------------------------------------------------------------------------------
    $(
        #[doc = "Static parser for " $name]
        pub struct [<$name Parser>] {
            re: Regex
        }

        impl [<$name Parser>] {
        #[doc = "Create enw instance of " $name " parser"]
            pub fn new(re: Regex) -> Self {
                Self { re }
            }

            /// Get captures from a text
            pub fn captures<'a>(&'a self, text: &'a str) -> Captures<'a> {
                self.re.captures(text).unwrap()
            }
            /// TODO: method to format output
            pub fn format<'a>(&'a self, captures: &'a Captures<'a>) -> String {
                format!("{:#?}", captures)
            }
            /// Get struct representation
            pub fn data<'a>(&'a self, text: &'a str) -> [<$name Data>] {
                #[allow(unused_variables)]
                let captures = self.captures(text);
                [<$name Data>]  {
                    $($capture: captures[stringify!($capture)].to_string()),*

                }
            }
        }

        #[doc = "Data representation of " $name]
        pub struct [<$name Data>] {
            $(#[doc = $capture:upper]
              pub $capture: String
              ),*
        }
    )*

    // InnerParser -------------------------------------------------------------------------------------------
    /// ...
    pub enum InnerParser {
        $(
            #[doc = "..."]
            $name([<$name Parser>])
         ),*
    }

    impl InnerParser {
        /// Get captures regardless of match type
        pub fn captures<'a>(&'a self, text: &'a str) -> Captures<'a> {
            match self {
                $(Self::$name(v) => v.captures(text),)*

            }
        }

        /// Format text
        pub fn format<'a>(&'a self, text: &'a str) -> String {
            match self {
                $(Self::$name(v) => v.format(&v.captures(text)),)*

            }
        }

    }


    // Matcher ----------------------------------------------------------------------------------------------
    /// Regex Set to match against
    pub struct Matcher {
        inner: RegexSet,
    }

    impl Matcher {
        /// Return regex reference to matching pattern.
        /// If multiple matches found, then it will print error for now with the pattern that this
        /// applied.
        pub fn get_parser_for(&self, text: &str) -> Option<&'static InnerParser> {
            let matches = self.inner.matches(text);
            if !matches.matched_any() {
                tracing::warn!("No match for `{text}`");
                return None;
            }

            if matches.len() > 1 {
                let patterns = matches
                    .iter()
                    .map(|idx| self.inner.patterns().get(idx).unwrap());

                tracing::error!(
                    "Multiple matches for {text}\n\nmatching patterns {:#?}",
                    patterns
                    );
            };

            for match_idx in matches.iter() {
                if let Some(matched) = PARSERS.get(match_idx) {
                    return Some(matched);
                }
            }
            None
        }

    }

    // Statics -----------------------------------------------------------------------------------------------
    lazy_static! {
        /// All Regex matchers
        pub static ref PARSERS: Vec<InnerParser> = vec![
            $(InnerParser::$name([<$name Parser>]::new(Regex::new($pattern).unwrap() ))),*
        ];

        /// Main Matcher for `PARSERS`
        pub static ref MATCHER: Matcher = {
            let mut patterns = vec![];
            $(patterns.push($pattern));*;
            let inner = RegexSet::new(&patterns).unwrap();
            Matcher {inner}
        };
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
