#![doc(html_root_url = "https://docs.rs/img-parts/0.1.1")]
// TODO: remove this once the MSRV is 1.42.0
#![allow(clippy::match_like_matches_macro)]

//! # img-parts
//!
//! The `img-parts` crate provides a low level api for reading and
//! writing containers from various image formats.
//!
//! It currently supports [`Jpeg`][crate::jpeg::Jpeg] and
//! [`RIFF`][crate::riff::RiffChunk] (with some helper functions
//! for [`WebP`][crate::webp::WebP]).
//!
//! With it you can read an image, modify its sections and save it
//! back.
//!
//! ```rust,no_run
//! # use std::fs::{self, File};
//! # use img_parts::Result;
//! # fn run() -> Result<()> {
//! use img_parts::jpeg::Jpeg;
//! use img_parts::{ImageEXIF, ImageICC};
//!
//! # let another_icc_profile = Vec::new();
//! # let new_exif_metadata = Vec::new();
//! let input = fs::read("img.jpg")?;
//! let output = File::create("out.jpg")?;
//!
//! let mut jpeg = Jpeg::from_bytes(input.into())?;
//! let icc_profile = jpeg.icc_profile();
//! let exif_metadata = jpeg.exif();
//!
//! jpeg.set_icc_profile(Some(another_icc_profile.into()));
//! jpeg.set_exif(Some(new_exif_metadata.into()));
//! jpeg.encoder().write_to(output)?;
//! # Ok(())
//! # }
//! ```

pub use common::{ImageEXIF, ImageICC};
pub use encoder::{ImageEncoder, ImageEncoderReader};
pub use error::{Error, Result};
pub(crate) use exif::EXIF_DATA_PREFIX;

mod common;
mod encoder;
mod error;
mod exif;
pub mod jpeg;
pub mod riff;
pub(crate) mod util;
pub mod vp8;
pub mod webp;
