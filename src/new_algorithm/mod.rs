/*
구조변경 정리

- 런타임의 전체 Data를 2개의 파트로 나눔: header, view.
- 큰 데이터들은 Body에 저장함.
- view는 실제 구동에 필요한 모든 정보들을 저장함.
  - 작으며 자주 사용되는 데이터는 header에서 복사해옴.
  - 큰 데이터는 Body에 대한 참조로.
- 즉, 디스크의 구조는 header + body이고, 메모리의 구조체는 header + view이다.

blob = header + body
┌────────────┬──────────────────────────┐
│  header    │          body            │
└────────────┴──────────────────────────┘

- Header의 역할:
  - 빌드에 필요한 옵션들 미리 저장하고, 필요한 blob 사이즈 계산가능.
  - 빌드하면서 Blob에 데이터를 쓰기 가능.
  - 빌드된 데이터를 blob에서 불러와서 view로 만들어줄 수 있음.
- ChrEncodingTable은 chr -> idx를 담고 있는 가벼운 구조체로, header로 구분되지만, 조건부로 view로도 활용 가능.

- 주의!: zerocopy를 위해서 모든 header와 body의 경계는 8바이트여야함.
  - Vector가 u128인 경우, 16바이트 경계를 체크해야하기 때문에, BWM의 rank_checkpoints만 16바이트 경계를 체크함.
*/

use crate::Position;

trait Header: zerocopy::FromBytes + zerocopy::IntoBytes + zerocopy::Immutable + zerocopy::KnownLayout + Sized{
    fn aligned_size(&self) -> usize {
        let raw_size = self.as_bytes().len();
        let rem = raw_size % 8;
        if rem == 0 { raw_size } else { raw_size + (8 - rem) }
    }
    fn write_to_blob(&self, blob: &mut [u8]) {
        self.write_to_prefix(blob).unwrap();
    }
    fn read_from_blob<'a>(blob: &'a [u8]) -> (Self, &'a [u8]) {
        let (header, _) = Self::read_from_prefix(blob).unwrap();
        let remaining_bytes = &blob[header.aligned_size()..];
        (header, remaining_bytes)
    }
}

trait View<'a> {
    type Header;

    fn aligned_body_size(header: &Self::Header) -> usize;
    fn load_from_body(
        header: &Self::Header,
        body_blob: &'a [u8],
    ) -> Self;
}

fn calculate_byte_size_with_padding(size: usize) -> usize {
    let rem = size % 8;
    if rem == 0 { size } else { size + (8 - rem) }
}

// Components of FM-index
mod magic_number;
use magic_number::MagicNumber;
mod encoding_table;
use encoding_table::ChrEncodingTable;
mod count_array;
use count_array::{CountArrayHeader, CountArrayView};
mod suffix_array;
use suffix_array::{SuffixArrayHeader, SuffixArrayView};
mod bwm;
use bwm::{BwmHeader, BwmView};
pub use bwm::{Block, blocks};

// Builder
mod builder;
pub use builder::{FmIndexBuilder, BuildError};

// Load from blob
mod load_from_blob;
pub use load_from_blob::LoadError;
// Core functionality
mod locate;

#[derive(Clone, PartialEq, Eq)]
pub struct FmIndex<'a, P: Position, B: Block> {
    // headers
    magic_number: MagicNumber,
    chr_encoding_table: ChrEncodingTable,
    count_array_header: CountArrayHeader,
    suffix_array_header: SuffixArrayHeader,
    bwm_header: BwmHeader,
    // views
    count_array_view: CountArrayView<'a, P>,
    suffix_array_view: SuffixArrayView<'a, P>,
    bwm_view: BwmView<'a, P, B>,
}
