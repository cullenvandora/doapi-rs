// id             number            A unique numeric ID that can be used to
// identify and reference an action.
// status         string            The current status of the action. This can
// be "in-progress", "completed", or "errored".
// type           string            This is the type of action that the object
// represents. For example, this could be "transfer" to represent the state of
// an image transfer action.
// started_at     string            A time value given in ISO8601 combined date
// and time format that represents when the action was initiated.
// completed_at   nullable string   A time value given in ISO8601 combined date
// and time format that represents when the action was completed.
// resource_id    number            A unique identifier for the resource that
// the action is associated with.
// resource_type  string            The type of resource that the action is
// associated with.
// region         nullable string   (deprecated) A slug representing the region
// where the action occurred.
// region_slug    nullable string   A slug representing the region where the
// action occurred.

use std::borrow::Cow;
use std::fmt;

use crate::response::{self, region::Region, NamedResponse};

#[derive(Deserialize, Debug)]
pub struct Action {
    pub id: f64,
    pub status: String,
    #[serde(rename = "type")]
    pub action_type: String,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub resource_id: f64,
    pub resource_type: String,
    pub region: Region,
    pub region_slug: Option<String>,
}

impl response::NotArray for Action {}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: {}\n\
                  Status: {}\n\
                  Type: {}\n\
                  Started At: {}\n\
                  Completed At: {}\n\
                  Resource ID: {}\n\
                  Resource Type: {}\n\
                  Region Slug: {}\n\
                  Region:\n\t{}",
            self.id,
            self.status,
            self.action_type,
            self.started_at,
            if self.completed_at.is_some() {
                self.completed_at.clone().unwrap()
            } else {
                "None".to_owned()
            },
            self.resource_id,
            self.resource_type,
            if self.region_slug.is_some() {
                self.region_slug.clone().unwrap()
            } else {
                "None".to_owned()
            },
            self.region.to_string().replace("\n", "\n\t")
        )
    }
}

pub type Actions = Vec<Action>;

impl NamedResponse for Action {
    fn name<'a>() -> Cow<'a, str> {
        "action".into()
    }
}
