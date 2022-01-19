use super::{
    Result, error_msg,
    Archive, Serialize, Deserialize,
    Text, Pattern,
    LtFmIndexConstructor, LtFmIndexInterface,
};

use super::{
    SelfDescLtFmIndexPreBuild, SelfDescLtFmIndex,
    TextType, BwtCompressionSize,
};

use rkyv::ser::{
    Serializer,
    serializers::AllocSerializer,
};

type DefaultSerializer = AllocSerializer::<0>;