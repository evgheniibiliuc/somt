use mockall::Any;

pub trait Logger {
    fn log(&self, msg: &str);
}

impl std::fmt::Debug for dyn Logger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Logger {{ {:?} }}", self.type_name())
    }
}