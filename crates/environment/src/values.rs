#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RuntimeValue {
    Number(f64),
    Null,
}
