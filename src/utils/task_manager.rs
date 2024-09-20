use std::sync::Arc;
use anyhow::Error;

struct TaskManager {
    tasks: Vec<Arc<dyn FnOnce() -> Result<(), Error>>>
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: vec![],
        }
    }
    
    pub fn new_task<F: FnOnce() -> Result<(), Error>>(&self, func: F) {
        
    }    
}