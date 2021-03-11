use url::{ParseError, Url};

pub trait Extend {
    fn extend(&self, input: &str) -> Result<Url, ParseError>;
}

impl Extend for Url {
    fn extend(&self, input: &str) -> Result<Url, ParseError> {
        Url::parse(&format!("{}/{}", self.as_str(), input))
    }
}
