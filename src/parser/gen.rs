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
    use super::{XCOutput, XCOutputTask};
    use anyhow::{Result, anyhow};

    lazy_static! {
        /// Main Matcher for `PARSERS`
        pub static ref XCLOG_MATCHER: XCLogMatcher = XCLogMatcher::default();
    }

    $(
        #[doc = $name "Captures" "created by `" $name "Parser`" ]
        pub struct [<XC $name Match>]<'a> {
            _inner: RegexCaptures<'a>,
            kind: XCOutputTask,
        }

        impl<'a> [<XC $name Match>]<'a> {
            /// Pretty format
            pub fn output(&self) -> Result<Option<XCOutput>> {
                if $format.is_empty() {
                    return  Ok(None)
                }
                $(
                    #[allow(unused_variables)]
                    let $capture = &self._inner
                        .name(stringify!($capture))
                        .ok_or_else(|| anyhow!{
                            "\n\nMissing [`{}`] from captures\n\n{:#?}",
                            stringify!($capture),
                            self._inner
                        })?
                        .as_str();
                 )*
                let leading = match self.kind {
                    XCOutputTask::Error => "[Error] ",
                    XCOutputTask::Warning => "[Warning] ",
                    _ => "",

                };

                Ok(Some(XCOutput {
                    value: format!("{}{}", leading, format!($format)),
                    kind: self.kind.clone(),
                }))
            }

            #[doc = "Get data struct representation of `XC" $name "`"]
            pub fn as_data(&self) -> [<XC $name Data>] {
                [<XC $name Data>] { $($capture: self._inner[stringify!($capture)].to_string()),* }
            }
        }

        #[doc = "Parser for XC `" $name "`"]
        pub struct [<XC $name Parser>] { re: Regex }
        impl [<XC $name Parser>] {
        #[doc = "Create enw instance of XC" $name " parser"]
            pub fn new(re: Regex) -> Self { Self { re } }

            /// Get captures from a text
            pub fn captures<'a>(&'a self, text: &'a str) -> Option<[<XC $name Match>]<'a>> {
                self.re.captures(text)
                    .map(|captures| {
                        [<XC $name Match>] {
                            _inner: captures,
                            kind: XCOutputTask::$kind
                        }
                    })
            }
        }

        #[doc = "Data representation of `XC" $name "`"]
        #[derive(Debug)]
        pub struct [<XC $name Data>] { $(#[doc = $capture:upper] pub $capture: String),* }
    )*

    /// A enum with all possible matches
    pub enum XCMatch<'a> { $(#[doc = "XC" $name " Match "] $name([<XC $name Match>]<'a>)),* }
    impl<'a> XCMatch<'a> {
        /// Format capture as text
        pub fn output(&'a self) -> Result<Option<XCOutput>> {
            match self { $(Self::$name(v) => v.output(),)* }
        }

        /// Check whether match is error
        pub fn is_error(&'a self) -> bool {
            match self { $(Self::$name(_) => XCOutputTask::$kind.is_error(),)* }
        }

        /// Check whether match is warning
        pub fn is_task(&'a self) -> bool {
            match self { $(Self::$name(_) => XCOutputTask::$kind.is_task(),)* }
        }

        /// Check whether match is result
        pub fn is_result(&'a self) -> bool {
            match self { $(Self::$name(_) => XCOutputTask::$kind.is_result(),)* }
        }

        /// Check whether match is test
        pub fn is_test(&'a self) -> bool {
            match self { $(Self::$name(_) => XCOutputTask::$kind.is_test(),)* }
        }

        /// Check whether match is warning
        pub fn is_warning(&'a self) -> bool {
            match self { $(Self::$name(_) => XCOutputTask::$kind.is_warning(),)* }
        }

        $(
            #[doc = "Check whether Match is `XC" $name "Match`"]
            pub fn [<is_ $name:snake:lower>](&self) -> bool {
                matches!(self, XCMatch::$name(_))
            }

            #[doc = "Return some if Match is `XC" $name "Match`"]
            pub fn [<as_ $name:snake:lower>](&self) -> Option<&[<XC $name Match>]<'a>> {
                if let XCMatch::$name(m) = self { Some(m) } else { None }
            }

            #[doc = "Return `" $name "Data` if match is " $name]
            pub fn [<as_ $name:snake:lower _data>](&self) -> Option<[<XC $name Data>]> {
                if let XCMatch::$name(m) = self { Some(m.as_data()) } else { None }
            }

        )*
    }

    /// Collection of all supported parsers
    pub enum XCParser { $(#[doc = "..."] $name(&'static [<XC $name Parser>])),* }
    impl XCParser {
        pub(crate) fn capture<'a>(&'a self, text: &'a str) -> Option<XCMatch<'a>> {
            match self {
                $(Self::$name(v) => v.captures(text).map(XCMatch::$name),)*
            }
        }
    }

    $(
    lazy_static::lazy_static! {
            static ref [<XC_ $name:snake:upper _PARSER>]: [<XC $name Parser>] = [<XC $name Parser>]::new(Regex::new($pattern).unwrap());
    }
    )*

    /// Matchers Using a vector of [`XCParser`]
    pub struct XCLogMatcher { inner: Vec<XCParser> }

    impl Default for XCLogMatcher {
        fn default() -> Self {
            Self {
                inner: vec![$(XCParser::$name(&*[<XC_ $name:snake:upper _PARSER>])),*]
            }
        }
    }

    impl XCLogMatcher {

        /// Return [`XCMatch`] if any thing is matched
        pub fn capture<'a>(&'a self, text: &'a str) -> Option<XCMatch<'a>> {
            for parser in self.inner.iter() {
                let captures = parser.capture(text);
                if captures.is_some() {
                    return captures
                }
            }
            None
        }

        /// Return [`XCMatch`] if any thing is matched
        pub fn get_compile_command(&self, text: &str) -> Option<XCCompileCommandData> {
            let parser = &XC_COMPILE_COMMAND_PARSER;
            if parser.re.is_match(text) {
                return parser.captures(text).map(|m| m.as_data())
            }

            None
        }
    }

    #[cfg(test)]
    mod tests {
        use regex::{Captures, Regex};
        use lazy_static::lazy_static;
        fn run_tests(captures: Captures, testfn: impl FnOnce(Captures)) {
            testfn(captures)
        }

        $(
            lazy_static! { static ref [<XC_ $name:snake:upper>]: Regex = Regex::new($pattern).unwrap(); }

            #[test]
            fn [<$name:snake:lower>]() {
                $(
                    let captures = match [<XC_ $name:snake:upper>].captures($test_value) {
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
}};}

pub(crate) use define;
