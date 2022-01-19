use crate::core::{
    Result, error_msg,
    Archive, Serialize, Deserialize,
    Text, Pattern,
    LtFmIndexConstructor, LtFmIndexInterface,
};
use crate::basic_structure::{
    RawLtFmIndexPreBuild,
    RawLtFmIndex,
    CountArrayConstructor, CountArrayInterface,
    BwtConstructor, BwtInterface,
};
use crate::text_type::{
    LtFmIndexPreBuild64NO, LtFmIndexPreBuild128NO, LtFmIndexPreBuild64NN, LtFmIndexPreBuild128NN,
    LtFmIndexPreBuild64AO, LtFmIndexPreBuild128AO, LtFmIndexPreBuild64AN, LtFmIndexPreBuild128AN,
};

use std::marker::PhantomPinned;
use std::pin::Pin;
use rkyv::ser::{
    Serializer,
    serializers::AllocSerializer,
};

type DefaultSerializer = AllocSerializer::<0>;

impl<C, B> RawLtFmIndexPreBuild<C, B> where
    C: CountArrayConstructor,
    B: BwtConstructor,
{
    
}

pub struct LtFmIndexWrapper<L> where
    L: LtFmIndexConstructor + Archive + Serialize<DefaultSerializer>,
    L::Archived: LtFmIndexInterface,
{
    pinned_bytes: Pin<Box<Vec<u8>>>,
    casted_pointer: *const L::Archived,
    _phantom_pinned: PhantomPinned,
}



impl<L> LtFmIndexWrapper<L> where
    L: LtFmIndexConstructor + Archive + Serialize<DefaultSerializer>,
    L::Archived: LtFmIndexInterface,
{
    fn wrapping(lt_fm_index_pre_build: &L) -> Self {
        let mut serializer = DefaultSerializer::default();
        let position = serializer.serialize_value(lt_fm_index_pre_build).unwrap();
        println!("pos: {}", position);
        let mut bytes = serializer.into_serializer().into_inner().to_vec();
        println!("len: {}", bytes.len());
        println!("cap: {}", bytes.capacity());

        let pinned_boxed_bytes = Pin::new(Box::new(bytes));

        let mut casted_pointer = std::ptr::null();
        casted_pointer = unsafe { rkyv::archived_value::<L>(&pinned_boxed_bytes, position) };

        Self {
            pinned_bytes: pinned_boxed_bytes,
            casted_pointer: casted_pointer,
            _phantom_pinned: PhantomPinned,
        }
    }
}

impl<L> LtFmIndexInterface for LtFmIndexWrapper<L> where
    L: LtFmIndexConstructor + Archive + Serialize<DefaultSerializer>,
    L::Archived: LtFmIndexInterface,
{
    fn count(&self, pattern: Pattern) -> u64 {
        unsafe{ self.casted_pointer.as_ref() }.unwrap().count(pattern)
    }
    fn locate(&self, pattern: Pattern) -> Vec<u64> {
        unsafe{ self.casted_pointer.as_ref() }.unwrap().locate(pattern)
    }
}

#[test]
fn test_ser_and_deser() {
    let text_1 = b"ATTTTTTTATTTTTTTATTTTTTTATTTTTTTATTTTTTT".to_vec();
    let pattern_1 = b"ATTT";

    let lt_fm_index = LtFmIndexPreBuild64NO::new(text_1, 2, 4);
    let mut wrapper_1 = LtFmIndexWrapper::wrapping(&lt_fm_index);

    let count = wrapper_1.count(pattern_1);
    println!("{:?}", count);
    let locate = wrapper_1.locate(pattern_1);
    println!("{:?}", locate);

    let text_2 = b"TTTTTATTTTTTTTTTTTTTATTTTTTTTTTTTTTATTTTTTTTTTTTTTATTTTTTTTT".to_vec();
    let pattern_2 = b"TTTTTTTTTTTTTT";

    let lt_fm_index = LtFmIndexPreBuild64NO::new(text_2, 2, 4);
    let mut wrapper_2 = LtFmIndexWrapper::wrapping(&lt_fm_index);

    let count = wrapper_2.count(pattern_2);
    println!("{:?}", count);
    let locate = wrapper_2.locate(pattern_2);
    println!("{:?}", locate);

    std::mem::swap(&mut wrapper_1, &mut wrapper_2);

    let count = wrapper_1.count(pattern_1);
    println!("{:?}", count);
    let locate = wrapper_1.locate(pattern_1);
    println!("{:?}", locate);
    let count = wrapper_2.count(pattern_2);
    println!("{:?}", count);
    let locate = wrapper_2.locate(pattern_2);
    println!("{:?}", locate);

    // let mut serializer = DefaultSerializer::default();
    // serializer.serialize_value(&lt_fm_index).unwrap();
    // let bytes = serializer.into_serializer().into_inner();

    // let test = bytes.to_vec();

    // let archived  = unsafe { rkyv::archived_root::<LtFmIndex64NO>(&bytes[..]) };

    // let pattern = b"ATTT";

    // let count = archived.count(pattern);
    // println!("{:?}", count);
    // let locate = archived.locate(pattern);
    // println!("{:?}", locate);
}