//! A module to provide functions for JSON <-> Rust serialize and deserialize.
//!
//! *This module is available if syn-serde is built with the `"json"` feature.*

use super::*;
use serde_json::Result;
use std::io;

// Serialize [`Syn`] type into JSON data.

/// Serialize the given [`Syn`] type as JSON into the IO stream.
///
/// This function is equivalent to the following code:
///
/// ```rust
/// # use std::io;
/// # fn to_writer<W>(writer: W, syn_file: &syn::File) -> serde_json::Result<()>
/// # where
/// #     W: io::Write,
/// # {
/// use syn_serde::Syn;
///
/// let serializable_file = syn_file.to_adapter();
/// serde_json::to_writer(writer, &serializable_file)
/// # }
/// ```
#[inline]
pub fn to_writer<S, W>(writer: W, syn: &S) -> Result<()>
where
    S: Syn,
    W: io::Write,
{
    let adapter = syn.to_adapter();
    serde_json::to_writer(writer, &adapter)
}

/// Serialize the given [`Syn`] type as pretty-printed JSON into the IO
/// stream.
///
/// This function is equivalent to the following code:
///
/// ```rust
/// # use std::io;
/// # fn to_writer_pretty<W: io::Write>(writer: W, syn_file: &syn::File) -> serde_json::Result<()> {
/// use syn_serde::Syn;
///
/// let serializable_file = syn_file.to_adapter();
/// serde_json::to_writer_pretty(writer, &serializable_file)
/// # }
/// ```
#[inline]
pub fn to_writer_pretty<S, W>(writer: W, syn: &S) -> Result<()>
where
    S: Syn,
    W: io::Write,
{
    let adapter = syn.to_adapter();
    serde_json::to_writer_pretty(writer, &adapter)
}

/// Serialize the given [`Syn`] type as a JSON byte vector.
///
/// This function is equivalent to the following code:
///
/// ```rust
/// # fn to_vec(syn_file: &syn::File) -> serde_json::Result<Vec<u8>> {
/// use syn_serde::Syn;
///
/// let serializable_file = syn_file.to_adapter();
/// serde_json::to_vec(&serializable_file)
/// # }
/// ```
#[inline]
pub fn to_vec<S>(syn: &S) -> Result<Vec<u8>>
where
    S: Syn,
{
    let adapter = syn.to_adapter();
    serde_json::to_vec(&adapter)
}

/// Serialize the given [`Syn`] type as a pretty-printed JSON byte vector.
///
/// This function is equivalent to the following code:
///
/// ```rust
/// # fn to_vec_pretty(syn_file: &syn::File) -> serde_json::Result<Vec<u8>> {
/// use syn_serde::Syn;
///
/// let serializable_file = syn_file.to_adapter();
/// serde_json::to_vec_pretty(&serializable_file)
/// # }
/// ```
#[inline]
pub fn to_vec_pretty<S>(syn: &S) -> Result<Vec<u8>>
where
    S: Syn,
{
    let adapter = syn.to_adapter();
    serde_json::to_vec_pretty(&adapter)
}

/// Serialize the given [`Syn`] type as a String of JSON.
///
/// This function is equivalent to the following code:
///
/// ```rust
/// # fn to_string(syn_file: &syn::File) -> serde_json::Result<String> {
/// use syn_serde::Syn;
///
/// let serializable_file = syn_file.to_adapter();
/// serde_json::to_string(&serializable_file)
/// # }
/// ```
#[inline]
pub fn to_string<S>(syn: &S) -> Result<String>
where
    S: Syn,
{
    let adapter = syn.to_adapter();
    serde_json::to_string(&adapter)
}

/// Serialize the given [`Syn`] type as a pretty-printed String of JSON.
///
/// This function is equivalent to the following code:
///
/// ```rust
/// # fn to_string_pretty(syn_file: &syn::File) -> serde_json::Result<String> {
/// use syn_serde::Syn;
///
/// let serializable_file = syn_file.to_adapter();
/// serde_json::to_string_pretty(&serializable_file)
/// # }
/// ```
#[inline]
pub fn to_string_pretty<S>(syn: &S) -> Result<String>
where
    S: Syn,
{
    let adapter = syn.to_adapter();
    serde_json::to_string_pretty(&adapter)
}

// Deserialize JSON data to [`Syn`] type.

/// Deserialize an instance of [`Syn`] type from an IO stream of JSON.
///
/// This function is equivalent to the following code:
///
/// ```rust
/// # use std::io;
/// # fn from_reader<R: io::Read>(rdr: R) -> serde_json::Result<syn::File> {
/// use syn_serde::Syn;
///
/// let serializable_file: <syn::File as Syn>::Adapter = serde_json::from_reader(rdr)?;
/// let syn_file = syn::File::from_adapter(&serializable_file);
/// Ok(syn_file)
/// # }
/// ```
pub fn from_reader<S, R>(rdr: R) -> Result<S>
where
    S: Syn,
    R: io::Read,
{
    let adapter: S::Adapter = serde_json::from_reader(rdr)?;
    Ok(S::from_adapter(&adapter))
}

/// Deserialize an instance of [`Syn`] type from bytes of JSON text.
///
/// This function is equivalent to the following code:
///
/// ```rust
/// # fn from_reader(v: &[u8]) -> serde_json::Result<syn::File> {
/// use syn_serde::Syn;
///
/// let serializable_file: <syn::File as Syn>::Adapter = serde_json::from_slice(v)?;
/// let syn_file = syn::File::from_adapter(&serializable_file);
/// Ok(syn_file)
/// # }
/// ```
pub fn from_slice<S>(v: &[u8]) -> Result<S>
where
    S: Syn,
{
    let adapter: S::Adapter = serde_json::from_slice(v)?;
    Ok(S::from_adapter(&adapter))
}

/// Deserialize an instance of [`Syn`] type from a string of JSON text.
///
/// This function is equivalent to the following code:
///
/// ```rust
/// # fn from_str(s: &str) -> serde_json::Result<syn::File> {
/// use syn_serde::Syn;
///
/// let serializable_file: <syn::File as Syn>::Adapter = serde_json::from_str(s)?;
/// let syn_file = syn::File::from_adapter(&serializable_file);
/// Ok(syn_file)
/// # }
/// ```
pub fn from_str<S>(s: &str) -> Result<S>
where
    S: Syn,
{
    let adapter: S::Adapter = serde_json::from_str(s)?;
    Ok(S::from_adapter(&adapter))
}
