
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Value {
    Variable,
    Constant,
    Reference,
}

impl Default for Value {
    fn default() -> Self {
        Value::Variable
    }
}
