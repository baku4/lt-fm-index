use crate::core::{
    Result, error_msg,
    Archive, Serialize, Deserialize,
    Text, Pattern,
};
use crate::structure::{
    LtFmIndexShortPreBuild,
    TextEncoder,
    BwtBlockConstructor, BwtBlockInterface,
};

mod nc_only;
