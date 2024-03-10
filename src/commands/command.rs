pub trait Command: Send + Sync {
    fn execute(&self);
}
