pub use taskctx::TaskStack;
use alloc::vec::Vec;

/// A simple stack pool
pub struct StackPool {
    free_stacks: Vec<TaskStack>,
}

impl StackPool {
    /// Creates a new empty stack pool.
    pub const fn new() -> Self {
        Self {
            free_stacks: Vec::new(),
        }
    }

    /// Alloc a free stack from the pool.
    pub fn alloc(&mut self) -> TaskStack {
        self.free_stacks.pop().unwrap_or_else(|| {
            let stack = TaskStack::alloc(axconfig::TASK_STACK_SIZE);
            stack
        })
    }

    /// Recycle a stack to the pool.
    pub fn recycle(&mut self, stack: TaskStack) {
        self.free_stacks.push(stack);
    }

}