#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
    Number(f64),
    Null,
}
