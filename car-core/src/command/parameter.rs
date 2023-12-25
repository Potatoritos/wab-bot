#[derive(Clone, Debug, PartialEq)]
pub enum ParameterType {
    String,
    Int,
    Bool,
    Number,
}
#[derive(Clone, Debug, Default)]
pub struct ParameterChoice<T> {
    name: String,
    value: T,
}
impl<T> ParameterChoice<T> {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn value(&self) -> &T {
        &self.value
    }
}
impl ParameterChoice<i64> {
    pub fn new(name: impl Into<String>, value: i64) -> Self {
        let name = name.into();
        assert!(!name.is_empty() && name.len() <= 100);
        let value = value.into();
        Self { name, value }
    }
}
impl ParameterChoice<f64> {
    pub fn new(name: impl Into<String>, value: f64) -> Self {
        let name = name.into();
        assert!(!name.is_empty() && name.len() <= 100);
        let value = value.into();
        Self { name, value }
    }
}
impl ParameterChoice<String> {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        let name = name.into();
        let value = value.into();
        assert!(!name.is_empty() && name.len() <= 100);
        assert!(!value.is_empty() && value.len() <= 100);
        Self { name, value }
    }
}

#[derive(Debug)]
pub struct Parameter {
    name: String,
    description: String,
    kind: ParameterType,
    required: bool,
    choices_string: Vec<ParameterChoice<String>>,
    choices_int: Vec<ParameterChoice<i64>>,
    choices_number: Vec<ParameterChoice<f64>>,
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
    pub fn choices_string(&self) -> &Vec<ParameterChoice<String>> {
        &self.choices_string
    }
    pub fn choices_int(&self) -> &Vec<ParameterChoice<i64>> {
        &self.choices_int
    }
    pub fn choices_number(&self) -> &Vec<ParameterChoice<f64>> {
        &self.choices_number
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
    choices_string: Vec<ParameterChoice<String>>,
    choices_int: Vec<ParameterChoice<i64>>,
    choices_number: Vec<ParameterChoice<f64>>,
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
    pub fn choice_string(&mut self, choice: ParameterChoice<String>) -> &mut Self {
        self.choices_string.push(choice);
        self
    }
    pub fn choices_string(&mut self, choices: Vec<ParameterChoice<String>>) -> &mut Self {
        self.choices_string = choices;
        self
    }
    pub fn choices_int(&mut self, choices: Vec<ParameterChoice<i64>>) -> &mut Self {
        self.choices_int = choices;
        self
    }
    pub fn choice_int(&mut self, choice: ParameterChoice<i64>) -> &mut Self {
        self.choices_int.push(choice);
        self
    }
    pub fn choices_number(&mut self, choices: Vec<ParameterChoice<f64>>) -> &mut Self {
        self.choices_number = choices;
        self
    }
    pub fn choice_number(&mut self, choice: ParameterChoice<f64>) -> &mut Self {
        self.choices_number.push(choice);
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
        assert!(self.choices_string.len() <= 25);
        assert!(self.choices_int.len() <= 25);
        assert!(self.choices_number.len() <= 25);

        assert!(self.choices_string.len() == 0 || self.kind == Some(ParameterType::String));
        assert!(self.choices_int.len() == 0 || self.kind == Some(ParameterType::Int));
        assert!(self.choices_number.len() == 0 || self.kind == Some(ParameterType::Number));

        Parameter {
            name: self.name.clone(),
            description: self.description.clone(),
            kind: self.kind.clone().unwrap(),
            required: self.required,
            choices_string: self.choices_string.clone(),
            choices_int: self.choices_int.clone(),
            choices_number: self.choices_number.clone(),
            min_value_int: self.min_value_int,
            max_value_int: self.max_value_int,
            min_value_number: self.min_value_number,
            max_value_number: self.max_value_number,
            min_length: self.min_length,
            max_length: self.max_length,
        }
    }
}
