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

type DefaultSerializer = AllocSerializer::<256>;

pub fn serialize_with_default_serializer<V>(value: &V) -> Vec<u8> where
    V: Archive + Serialize<DefaultSerializer>
{
    let mut serializer = DefaultSerializer::default();
    serializer.serialize_value(value).unwrap();
    serializer.into_serializer().into_inner().to_vec()
}
