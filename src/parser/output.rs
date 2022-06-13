/// Kinds of outputs
#[derive(derive_is_enum_variant::is_enum_variant, Clone)]
pub enum OutputKind {
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
#[derive(derive_deref_rs::Deref)]
pub struct MatchOutput {
    #[deref]
    pub(crate) value: Option<String>,
    pub(crate) kind: OutputKind,
}

impl MatchOutput {
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
