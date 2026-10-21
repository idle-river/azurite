#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeVal {
    FloatingNumber(f32),
    Integer(i32),
    Null,
}
