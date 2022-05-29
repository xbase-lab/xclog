/// Kinds of outputs
pub enum OutputKind {
    Task,
    Test,
    Warning,
    Error,
    Result,
}

#[derive(derive_deref_rs::Deref)]
/// Formatted results of a given match
pub struct FormattedOutput {
    #[deref]
    pub value: Option<String>,
    pub kind: OutputKind,
}
