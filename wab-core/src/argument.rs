use twilight_model::application::interaction::application_command::CommandOptionValue;

pub enum Argument {
    String(String),
    Integer(i64),
    Boolean(bool),
    Float(f64),
}

impl TryFrom<&CommandOptionValue> for Argument {
    type Error = ();
    fn try_from(v: &CommandOptionValue) -> Result<Self, Self::Error> {
        match v {
            CommandOptionValue::Boolean(x) => Ok(Self::Boolean(*x)),
            CommandOptionValue::Integer(x) => Ok(Self::Integer(*x)),
            CommandOptionValue::Number(x) => Ok(Self::Float(*x)),
            CommandOptionValue::String(x) => Ok(Self::String(x.to_string())),
            _ => Err(()),
        }
    }
}