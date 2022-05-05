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
