mod serializer;
mod build_save_load;
mod casting_vs_including;

// Bench serializer 
pub use serializer::{
    bench_serialization_btw_serializer,
};

// Bench build, save and loadd
pub use build_save_load::{
    bench_build_arc_vs_unarc,
    bench_save_arc_vs_unarc,
    bench_load_arc_vs_unarc,
    bench_locate_arc_vs_unarc,
};

// Bench Casting vs Including
pub use casting_vs_including::{
    bench_save_taking_vs_writing,
    bench_load_casting_vs_including,
};
