pub trait Game<'a>
where
    <Self as Game<'a>>::Params: 'a + GameParams,
{
    type Params;
    const NAME: &'a str;
    const WINHELP_TOPIC: &'a str;
    const HTMLHELP_TOPIC: &'a str;
    const PRESETS: &'a [(&'a str, Self::Params)];
}

pub trait GameParams: Default + Clone {
    const CAN_CONFIGURE: bool;
    fn decode(encoded: &str) -> Self;
    fn encode(&self) -> String;
    fn encode_full(&self) -> String;
    fn configure(&self) -> Vec<ConfigItem>;
    fn custom_params(cfg: &[ConfigItem]) -> Option<Self>;
    fn validate(&self) -> Result<(), &'static str>;
}

pub struct ConfigItem {
    pub name: String,
    pub value: ConfigItemType,
}

pub enum ConfigItemType {
    String(String),
    Choices { names: Vec<String>, selected: usize },
    Bool(bool),
}

impl ConfigItemType {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(string) => Some(string),
            _ => None
        }
    }

    pub fn as_choice(&self) -> Option<usize> {
        match self {
            Self::Choices { selected, .. } => Some(*selected),
            _ => None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(bool) => Some(*bool),
            _ => None
        }
    }
}
