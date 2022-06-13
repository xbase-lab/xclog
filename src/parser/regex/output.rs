/// Kinds of outputs
#[derive(derive_is_enum_variant::is_enum_variant, Clone)]
pub enum OutputKind {
    Task,
    Test,
    Warning,
    Error,
    Result,
}

/// Formatted results of a given match
#[derive(derive_deref_rs::Deref)]
pub struct MatchOutput {
    #[deref]
    pub value: Option<String>,
    pub kind: OutputKind,
}

impl MatchOutput {
    pub fn is_task(&self) -> bool {
        self.kind.is_task()
    }

    pub fn is_error(&self) -> bool {
        self.kind.is_error()
    }

    pub fn is_test(&self) -> bool {
        self.kind.is_test()
    }

    pub fn is_result(&self) -> bool {
        self.kind.is_result()
    }
}
