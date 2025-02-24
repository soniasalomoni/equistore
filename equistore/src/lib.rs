//! # Equistore
//!
//! Equistore is a library providing a specialized data storage format for
//! atomistic machine learning (think ``numpy`` or ``torch.Tensor``, but also
//! carrying extra metadata for atomistic systems).
//!
//! The core of the format is implemented in `equistore-core`, and exported as a
//! C API. This C API is then re-exported to Rust in this crate. By default,
//! equistore-core is distributed as a shared library that you'll need to
//! install separately on end user machines.
//!
//! ## Features
//!
//! You can enable the `static` feature in Cargo.toml to use a static build of
//! the C API, removing the need to carry around the equistore-core shared
//! library.
//!
//! ```toml
//! [dependencies]
//! equistore = {version = "...", features = ["static"]}
//! ```

#![warn(clippy::all, clippy::pedantic)]

// disable some style lints
#![allow(clippy::needless_return, clippy::must_use_candidate, clippy::comparison_chain)]
#![allow(clippy::redundant_field_names, clippy::redundant_closure_for_method_calls, clippy::redundant_else)]
#![allow(clippy::unreadable_literal, clippy::option_if_let_else, clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc, clippy::missing_safety_doc)]
#![allow(clippy::similar_names, clippy::borrow_as_ptr, clippy::uninlined_format_args)]

#[cfg(all(test, static_and_rustc_older_1_63))]
fn fail_build() {
    // We need rustc>=1.63 for tests because of
    // https://github.com/rust-lang/rust/issues/100066
    compile_error!("the 'static' feature requires rustc>=1.63 for tests");
}

pub mod c_api;

pub mod errors;
pub use self::errors::Error;

mod data;
pub use self::data::{ArrayRef, ArrayRefMut};
pub use self::data::{Array, EmptyArray};

mod labels;
pub use self::labels::{Labels, LabelsBuilder, LabelValue};
pub use self::labels::{LabelsIter, LabelsFixedSizeIter};

#[cfg(feature = "rayon")]
pub use self::labels::LabelsParIter;

mod block;
pub use self::block::{TensorBlock, TensorBlockRef, TensorBlockRefMut};
pub use self::block::{BasicBlock, BasicBlockMut};
pub use self::block::{GradientsIter, GradientsMutIter};

mod tensor;
pub use self::tensor::TensorMap;
pub use self::tensor::{TensorMapIter, TensorMapIterMut};
#[cfg(feature = "rayon")]
pub use self::tensor::{TensorMapParIter, TensorMapParIterMut};

pub mod io;


/// Path where the equistore shared library has been built
pub fn c_api_install_dir() -> &'static str {
    return env!("OUT_DIR");
}
