#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeVal {
    FloatingNumber(f64),
    Integer(i32),
    Null,
}

impl RuntimeVal {
    fn as_f64(&self) -> Option<f64> {
        match self {
            RuntimeVal::Integer(i) => Some(*i as f64),
            RuntimeVal::FloatingNumber(f) => Some(*f),
            RuntimeVal::Null => None,
        }
    }
}
