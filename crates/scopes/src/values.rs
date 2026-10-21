#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeVal {
    FloatingNumber(f64),
    Boolean(bool),
    Integer(i32),
    Null,
}
