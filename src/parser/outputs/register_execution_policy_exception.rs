use std::fmt::Display;

use tap::Pipe;

use crate::parser::{Description, Error};

/// Register Execution policy
#[derive(Debug)]
pub struct RegisterExecutionPolicyException {
    pub description: Description,
}

impl RegisterExecutionPolicyException {
    pub fn new(line: String) -> Result<Self, Error> {
        Self {
            description: Description::from_line(line)?,
        }
        .pipe(Ok)
    }
}

impl Display for RegisterExecutionPolicyException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} Registering Execution Policy ...", self.description,)
    }
}
