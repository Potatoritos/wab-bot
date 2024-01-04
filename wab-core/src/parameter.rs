use twilight_model::application::command::{
    CommandOptionChoice, CommandOptionChoiceValue, CommandOptionType, CommandOptionValue,
};

#[derive(Clone, Debug, PartialEq)]
pub enum ParameterType {
    String,
    Integer,
    Boolean,
    Float,
}
impl ParameterType {
    pub fn create_twilight_option_type(&self) -> CommandOptionType {
        match self {
            Self::String => CommandOptionType::String,
            Self::Integer => CommandOptionType::Integer,
            Self::Float => CommandOptionType::Number,
            Self::Boolean => CommandOptionType::Boolean,
        }
    }
}
#[derive(Clone, Debug)]
pub enum ParameterChoiceType {
    String(String),
    Integer(i64),
    Float(f64),
}
#[derive(Clone, Debug)]
pub struct ParameterChoice {
    name: String,
    value: ParameterChoiceType,
}
impl ParameterChoice {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn value(&self) -> &ParameterChoiceType {
        &self.value
    }
    pub fn create_twilight_choice(&self) -> CommandOptionChoice {
        let value = match &self.value {
            ParameterChoiceType::Integer(v) => CommandOptionChoiceValue::Integer(v.clone()),
            ParameterChoiceType::Float(v) => CommandOptionChoiceValue::Number(v.clone()),
            ParameterChoiceType::String(v) => CommandOptionChoiceValue::String(v.clone()),
        };
        CommandOptionChoice {
            name: self.name.clone(),
            name_localizations: None,
            value,
        }
    }
    pub fn new(name: impl Into<String>, value: ParameterChoiceType) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}

#[derive(Debug)]
pub struct Parameter {
    name: String,
    description: String,
    kind: ParameterType,
    required: bool,
    choices: Vec<ParameterChoice>,
    min_value_int: Option<i64>,
    max_value_int: Option<i64>,
    min_value_number: Option<f64>,
    max_value_number: Option<f64>,
    min_length: Option<i32>,
    max_length: Option<i32>,
}

impl Parameter {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn description(&self) -> &str {
        self.description.as_str()
    }
    pub fn kind(&self) -> &ParameterType {
        &self.kind
    }
    pub fn required(&self) -> &bool {
        &self.required
    }
    pub fn choices(&self) -> &Vec<ParameterChoice> {
        &self.choices
    }
    pub fn min_value_int(&self) -> &Option<i64> {
        &self.min_value_int
    }
    pub fn min_value_number(&self) -> &Option<f64> {
        &self.min_value_number
    }
    pub fn max_value_int(&self) -> &Option<i64> {
        &self.max_value_int
    }
    pub fn max_value_number(&self) -> &Option<f64> {
        &self.max_value_number
    }
    pub fn min_length(&self) -> &Option<i32> {
        &self.min_length
    }
    pub fn max_length(&self) -> &Option<i32> {
        &self.max_length
    }
    pub fn create_twilight_choices(&self) -> Option<Vec<CommandOptionChoice>> {
        if self.choices.is_empty() {
            return None;
        }
        Some(
            self.choices
                .iter()
                .map(|c| c.create_twilight_choice())
                .collect(),
        )
    }
    pub fn create_twilight_max_value(&self) -> Option<CommandOptionValue> {
        if let Some(value) = self.max_value_int {
            Some(CommandOptionValue::Integer(value.clone()))
        } else if let Some(value) = self.max_value_number {
            Some(CommandOptionValue::Number(value.clone()))
        } else {
            None
        }
    }
    pub fn create_twilight_min_value(&self) -> Option<CommandOptionValue> {
        if let Some(value) = self.min_value_int {
            Some(CommandOptionValue::Integer(value.clone()))
        } else if let Some(value) = self.min_value_number {
            Some(CommandOptionValue::Number(value.clone()))
        } else {
            None
        }
    }
    pub fn create_twilight_max_length(&self) -> Option<u16> {
        if let Some(len) = self.max_length {
            Some(u16::try_from(len.clone()).unwrap())
        } else {
            None
        }
    }
    pub fn create_twilight_min_length(&self) -> Option<u16> {
        if let Some(len) = self.min_length {
            Some(u16::try_from(len.clone()).unwrap())
        } else {
            None
        }
    }
    pub fn builder() -> ParameterBuilder {
        ParameterBuilder::new()
    }
}

#[derive(Debug, Default)]
pub struct ParameterBuilder {
    name: String,
    description: String,
    kind: Option<ParameterType>,
    required: bool,
    choices: Vec<ParameterChoice>,
    min_value_int: Option<i64>,
    max_value_int: Option<i64>,
    min_value_number: Option<f64>,
    max_value_number: Option<f64>,
    min_length: Option<i32>,
    max_length: Option<i32>,
}
impl ParameterBuilder {
    fn new() -> Self {
        Self::default()
    }
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into();
        self
    }
    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        self.description = description.into();
        self
    }
    pub fn kind(&mut self, kind: ParameterType) -> &mut Self {
        self.kind = Some(kind);
        self
    }
    pub fn required(&mut self, required: bool) -> &mut Self {
        self.required = required;
        self
    }
    pub fn choice(&mut self, choice: ParameterChoice) -> &mut Self {
        self.choices.push(choice);
        self
    }
    pub fn choices(&mut self, choices: Vec<ParameterChoice>) -> &mut Self {
        self.choices = choices;
        self
    }
    pub fn min_value_int(&mut self, min_value_int: Option<i64>) -> &mut Self {
        self.min_value_int = min_value_int;
        self
    }
    pub fn min_value_number(&mut self, min_value_number: Option<f64>) -> &mut Self {
        self.min_value_number = min_value_number;
        self
    }
    pub fn max_value_int(&mut self, max_value_int: Option<i64>) -> &mut Self {
        self.max_value_int = max_value_int;
        self
    }
    pub fn max_value_number(&mut self, max_value_number: Option<f64>) -> &mut Self {
        self.max_value_number = max_value_number;
        self
    }
    pub fn min_length(&mut self, min_length: Option<i32>) -> &mut Self {
        self.min_length = min_length;
        self
    }
    pub fn max_length(&mut self, max_length: Option<i32>) -> &mut Self {
        self.max_length = max_length;
        self
    }
    pub fn build(&mut self) -> Parameter {
        assert!(!self.description.is_empty() && self.description.len() <= 100);
        assert!(!self.name.is_empty() && self.name.len() <= 32);

        if let Some(min_length) = self.min_length {
            assert!(0 <= min_length && min_length <= 6000);
        }
        if let Some(max_length) = self.max_length {
            assert!(0 <= max_length && max_length <= 6000);
        }
        if let (Some(min_value_int), Some(max_value_int)) = (self.min_value_int, self.max_value_int)
        {
            assert!(min_value_int <= max_value_int);
        }
        if let (Some(min_value_number), Some(max_value_number)) =
            (self.min_value_number, self.max_value_number)
        {
            assert!(min_value_number <= max_value_number);
        }
        if let (Some(min_length), Some(max_length)) = (self.min_length, self.max_length) {
            assert!(min_length <= max_length);
        }
        assert!(self.choices.len() <= 25);

        Parameter {
            name: self.name.clone(),
            description: self.description.clone(),
            kind: self.kind.clone().unwrap(),
            required: self.required,
            choices: self.choices.clone(),
            min_value_int: self.min_value_int,
            max_value_int: self.max_value_int,
            min_value_number: self.min_value_number,
            max_value_number: self.max_value_number,
            min_length: self.min_length,
            max_length: self.max_length,
        }
    }
}
