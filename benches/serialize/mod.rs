#[cfg(feature = "zero_copy")]
mod zero_copy_ser;
#[cfg(feature = "zero_copy")]
pub use zero_copy_ser::{
    bench_serialize_btw_zero_copy_serializers,
    bench_deserialize_btw_zero_copy_deserializers,
    bench_get_archived,
};

#[cfg(not(feature = "zero_copy"))]
mod serde_ser;
#[cfg(not(feature = "zero_copy"))]
pub use serde_ser::{
    bench_serialize_btw_bincode_serializers,
    bench_deserialize_btw_bincode_deserializers,
    bench_get_archived,
};