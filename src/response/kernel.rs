use std::fmt;
use std::borrow::Cow;

use response::NamedResponse;

#[derive(Deserialize, Debug)]
pub struct Kernel;

impl fmt::Display for Kernel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "kernel!")
    }
}

impl NamedResponse for Kernel {
    fn name<'a>() -> Cow<'a, str> {
        "kernel".into()
    }
}

pub type Kernels = Vec<Kernel>;