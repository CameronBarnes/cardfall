use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{BorrowedStr, StoredStr};

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
pub struct RelatedCard {
    id: Uuid,
    component: StoredStr,
    name: String,
    type_line: StoredStr,
    uri: String,
}

impl RelatedCard {
    pub const fn id(&self) -> Uuid {
        self.id
    }

    pub const fn component(&self) -> BorrowedStr {
        cfg_if! {
            if #[cfg(feature = "shared_str")] {
                self.component
            } else {
                &self.component
            }
        }
    }

    pub const fn type_line(&self) -> BorrowedStr {
        cfg_if! {
            if #[cfg(feature = "shared_str")] {
                self.type_line
            } else {
                &self.type_line
            }
        }
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }
}
