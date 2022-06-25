/// Kinds of outputs
#[derive(Clone, derive_is_enum_variant::is_enum_variant)]
pub enum XCOutputTask {
    /// Task like Compile, Mkdir ..
    Task,
    /// Test step or result
    Test,
    /// Warning
    Warning,
    /// Error
    Error,
    /// End Result
    Result,
}

/// Formatted results of a given match
#[derive(Clone, derive_deref_rs::Deref)]
pub struct XCOutput {
    #[deref]
    /// output value
    pub value: String,
    /// output kind
    pub kind: XCOutputTask,
}

impl XCOutput {
    /// Whether the output is a task
    pub fn is_task(&self) -> bool {
        self.kind.is_task()
    }

    /// Whether the output is an error
    pub fn is_error(&self) -> bool {
        self.kind.is_error()
    }

    /// Whether the output is a test
    pub fn is_test(&self) -> bool {
        self.kind.is_test()
    }

    /// Whether the output is a result
    pub fn is_result(&self) -> bool {
        self.kind.is_result()
    }
}

impl std::fmt::Display for XCOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}
