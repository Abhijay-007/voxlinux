
use crate::{intent::Intent, error::IntentError};

pub trait IntentExecutor {
    fn execute(&self, intent: &Intent) -> Result<(), IntentError>;
}
