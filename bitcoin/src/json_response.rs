use crate::BtcError;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as Json};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct JsonResponse {
    error: Option<Json>,
    result: Option<Json>,
}

impl JsonResponse {
    pub(crate) fn is_error(&self) -> bool {
        self.error.is_some()
    }

    pub(crate) fn is_result(&self) -> bool {
        self.result.is_some()
    }

    pub(crate) fn result(self) -> Result<Json, BtcError> {
        if let Some(j) = self.result {
            Ok(j)
        } else {
            Err(BtcError::HttpJsonResponse(
                self.error.expect("this to exist").to_string(),
            ))
        }
    }
}

impl fmt::Display for JsonResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", json!(self))
    }
}
