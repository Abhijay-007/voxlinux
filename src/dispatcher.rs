
use std::collections::HashMap;

use crate::{
    intent::{Intent, IntentType},
    executor::IntentExecutor,
    error::IntentError,
};

pub struct Dispatcher {
    executors: HashMap<IntentType, Box<dyn IntentExecutor>>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            executors: HashMap::new(),
        }
    }

    pub fn register<E>(&mut self, intent: IntentType, executor: E)
    where
        E: IntentExecutor + 'static,
    {
        self.executors.insert(intent, Box::new(executor));
    }

    pub fn dispatch(&self, intent: &Intent) -> Result<(), IntentError> {
        self.executors
            .get(&intent.intent_type)
            .ok_or(IntentError::ParseError)?
            .execute(intent)
    }
}
