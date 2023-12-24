pub enum Argument {
    String(String),
    OptionalString(Option<String>),
    Int(i64),
    OptionalInt(Option<i64>),
    Bool(bool),
    OptionalBool(Option<bool>),
    Number(f64),
    OptionalNumber(Option<f64>),
}
