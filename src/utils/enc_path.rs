use serde::{Deserialize, Serialize};
use crate::make_wrapped_db_type;

make_wrapped_db_type!(EncPath, String, Default, Serialize, Deserialize, Debug);