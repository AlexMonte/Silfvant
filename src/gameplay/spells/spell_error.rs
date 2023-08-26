pub trait SpellError {
    fn get_message(&self) -> String;
}

pub struct SpellCompileError {
    pub message: String,
}

impl SpellError for SpellCompileError {
    fn get_message(&self) -> String {
        self.message.clone()
    }
}

pub struct SpellRuntimeError {
    pub message: String,
}

impl SpellError for SpellRuntimeError {
    fn get_message(&self) -> String {
        self.message.clone()
    }
}
