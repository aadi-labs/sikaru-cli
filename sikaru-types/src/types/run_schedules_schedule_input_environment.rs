pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ScheduleInputEnvironment {
    #[serde(rename = "production")]
    Production,
}
impl fmt::Display for ScheduleInputEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Production => "production",
        };
        write!(f, "{}", s)
    }
}
