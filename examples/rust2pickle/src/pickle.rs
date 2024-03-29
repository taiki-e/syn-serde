// SPDX-License-Identifier: Apache-2.0 OR MIT

//! A module to provide functions for Pickle <-> Rust serialize and deserialize.

#![allow(unreachable_pub)]

use std::io;

use serde_pickle::{DeOptions, Result, SerOptions};
use syn_serde::Syn;

// Serialize [`Syn`] type into Pickle data.
/// Serialize the given [`Syn`] type as a pickle byte vector.
///
/// This function is equivalent to the following code:
///
/// ```
/// # fn to_vec(syn_file: &syn::File) -> Vec<u8> {
/// use syn_serde::Syn;
///
/// let serializable_file = syn_file.to_adapter();
/// serde_pickle::to_vec(&serializable_file, true).unwrap()
/// # }
/// ```
pub fn to_vec<S>(syn: &S) -> Vec<u8>
where
    S: Syn,
{
    let adapter = syn.to_adapter();
    serde_pickle::to_vec(&adapter, SerOptions::default()).unwrap()
}

// Deserialize JSON data to [`Syn`] type.

/// Deserialize an instance of [`Syn`] type from an I/O stream of JSON.
///
/// This function is equivalent to the following code:
///
/// ```
/// # use std::io;
/// # fn from_reader<R: io::Read>(rdr: R) -> serde_pickle::Result<syn::File> {
/// use syn_serde::Syn;
///
/// let serializable_file: <syn::File as Syn>::Adapter = serde_pickle::from_reader(rdr)?;
/// let syn_file = syn::File::from_adapter(&serializable_file);
/// Ok(syn_file)
/// # }
/// ```
#[allow(dead_code)]
pub fn from_reader<S, R>(rdr: R) -> Result<S>
where
    S: Syn,
    R: io::Read,
{
    let adapter: S::Adapter = serde_pickle::from_reader(rdr, DeOptions::default())?;
    Ok(S::from_adapter(&adapter))
}

/// Deserialize an instance of [`Syn`] type from bytes of JSON text.
///
/// This function is equivalent to the following code:
///
/// ```
/// # fn from_reader(v: &[u8]) -> serde_pickle::Result<syn::File> {
/// use syn_serde::Syn;
///
/// let serializable_file: <syn::File as Syn>::Adapter = serde_pickle::from_slice(v)?;
/// let syn_file = syn::File::from_adapter(&serializable_file);
/// Ok(syn_file)
/// # }
/// ```
#[allow(dead_code)]
pub fn from_slice<S>(v: &[u8]) -> Result<S>
where
    S: Syn,
{
    let adapter: S::Adapter = serde_pickle::from_slice(v, DeOptions::default())?;
    Ok(S::from_adapter(&adapter))
}
