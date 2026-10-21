#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeVal {
    FloatingNumber(f64),
    Integer(i32),
    Null,
}
