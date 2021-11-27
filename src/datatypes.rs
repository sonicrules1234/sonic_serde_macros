#[derive(Debug, Clone)]
pub struct DataType {
    pub name: String,
    pub stored_as: String,
    pub output_as: String,
    pub can_ref: bool,
    pub how_to_output: String,
    pub exclude_from_froms: bool,
    pub how_to_convert: String
}
impl DataType {
    pub fn new(name: impl Into<String>, stored_as: impl Into<String>) -> Self {
        let stored = stored_as.into();
        Self {
            name: name.into(),
            stored_as: stored.clone(),
            output_as: stored,
            can_ref: true,
            how_to_output: "x.clone()".to_string(),
            exclude_from_froms: false,
            how_to_convert: "newval".to_string(),
        }
    }
    pub fn can_ref(mut self, can_ref: bool) -> Self {
        self.can_ref = can_ref;
        self
    }
    pub fn output_as(mut self, output_type: impl Into<String>) -> Self {
        self.output_as = output_type.into();
        self
    }
    pub fn how_to_output(mut self, how_to_output: impl Into<String>) -> Self {
        self.how_to_output = how_to_output.into();
        self
    }
    pub fn exclude_from_froms(mut self, exclude_from_froms: bool) -> Self {
        self.exclude_from_froms = exclude_from_froms;
        self
    }
    pub fn how_to_convert(mut self, how_to_convert: impl Into<String>) -> Self {
        self.how_to_convert = how_to_convert.into();
        self
    }
}