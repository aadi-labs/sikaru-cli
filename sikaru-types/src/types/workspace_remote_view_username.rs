pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WorkspaceRemoteViewUsername {
    #[serde(rename = "x-token")]
    XToken,
}
impl fmt::Display for WorkspaceRemoteViewUsername {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::XToken => "x-token",
        };
        write!(f, "{}", s)
    }
}
