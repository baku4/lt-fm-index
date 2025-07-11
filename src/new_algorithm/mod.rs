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
*/

use crate::Position;
use zerocopy::{IntoBytes, Ref};

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
pub mod builder;

// Error
#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    /// Error while loading Fm-index headers.
    #[error("Error while loading Fm-index headers. Not a valid FM-index blob.")]
    InvalidHeaders,
    /// Blob size is not accurate.
    #[error("Invalid blob size. Expected: {0}, Actual: {1}")]
    InvalidBlobSize(usize, usize),
}

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

impl<'a, P: Position, B: Block> FmIndex<'a, P, B> {
    pub fn load(blob: &'a [u8]) -> Result<Self, LoadError> {
        // Load headers
        let (magic_number, remaining_bytes) = Ref::<_, MagicNumber>::from_prefix(blob)
            .map_err(|_| LoadError::InvalidHeaders)?;
        let (chr_encoding_table, remaining_bytes) = Ref::<_, ChrEncodingTable>::from_prefix(remaining_bytes)
            .map_err(|_| LoadError::InvalidHeaders)?;
        let (count_array_header, remaining_bytes) = Ref::<_, CountArrayHeader>::from_prefix(remaining_bytes)
            .map_err(|_| LoadError::InvalidHeaders)?;
        let (suffix_array_header, remaining_bytes) = Ref::<_, SuffixArrayHeader>::from_prefix(remaining_bytes)
            .map_err(|_| LoadError::InvalidHeaders)?;
        let (bwm_header, body_blob) = Ref::<_, BwmHeader>::from_prefix(remaining_bytes)
            .map_err(|_| LoadError::InvalidHeaders)?;

        // check body size
        let actual_body_size = body_blob.len();
        let expected_body_size = {
            count_array_header.calculate_body_size::<P>()
            + suffix_array_header.calculate_body_size::<P>()
            + bwm_header.calculate_body_size::<P, B>()
        };
        if actual_body_size != expected_body_size {
            let header_size = {
                magic_number.as_bytes().len()
                + chr_encoding_table.as_bytes().len()
                + count_array_header.as_bytes().len()
                + suffix_array_header.as_bytes().len()
                + bwm_header.as_bytes().len()
            };
            return Err(LoadError::InvalidBlobSize(
                header_size + expected_body_size,
                header_size + actual_body_size,
            ));
        }

        // Get views
        //  - Count array
        let mut body_start_index = 0;
        let mut body_end_index = count_array_header.calculate_body_size::<P>();
        let count_array_view = count_array_header.load::<P>(&body_blob[body_start_index..body_end_index]);
        //  - Suffix array
        body_start_index = body_end_index;
        body_end_index += suffix_array_header.calculate_body_size::<P>();
        let suffix_array_view = suffix_array_header.load::<P>(&body_blob[body_start_index..body_end_index]);
        //  - BWM
        body_start_index = body_end_index;
        body_end_index += bwm_header.calculate_body_size::<P, B>();
        let bwm_view = bwm_header.load::<P, B>(&body_blob[body_start_index..body_end_index]);

        Ok(Self {
            magic_number: Ref::read(&magic_number),
            chr_encoding_table: Ref::read(&chr_encoding_table),
            count_array_header: Ref::read(&count_array_header),
            suffix_array_header: Ref::read(&suffix_array_header),
            bwm_header: Ref::read(&bwm_header),
            count_array_view,
            suffix_array_view,
            bwm_view,
        })
    }
}
