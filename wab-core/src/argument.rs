pub enum Argument {
    String(String),
    OptionalString(Option<String>),
    Integer(i64),
    OptionalInteger(Option<i64>),
    Boolean(bool),
    OptionalBoolean(Option<bool>),
    Number(f64),
    OptionalNumber(Option<f64>),
}
