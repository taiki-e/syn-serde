//! A module to provide functions for JSON <-> Rust serialize and deserialize.

use std::io;

use serde_json::Result;

use super::*;

// Serialize [`Syn`] type into JSON data.

/// Serialize the given [`Syn`] type as JSON into the I/O stream.
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
/// let adapter = syn_file.to_adapter();
/// serde_json::to_writer(writer, &adapter)
/// # }
/// ```
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
/// let adapter = syn_file.to_adapter();
/// serde_json::to_writer_pretty(writer, &adapter)
/// # }
/// ```
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
/// # fn to_vec(syn_file: &syn::File) -> Vec<u8> {
/// use syn_serde::Syn;
///
/// let adapter = syn_file.to_adapter();
/// serde_json::to_vec(&adapter).unwrap()
/// # }
/// ```
// All of the data structures in syn-serde are compatible with JSON so unwrap will never fail.
#[allow(clippy::missing_panics_doc)]
pub fn to_vec<S>(syn: &S) -> Vec<u8>
where
    S: Syn,
{
    let adapter = syn.to_adapter();
    serde_json::to_vec(&adapter).unwrap()
}

/// Serialize the given [`Syn`] type as a pretty-printed JSON byte vector.
///
/// This function is equivalent to the following code:
///
/// ```rust
/// # fn to_vec_pretty(syn_file: &syn::File) -> Vec<u8> {
/// use syn_serde::Syn;
///
/// let adapter = syn_file.to_adapter();
/// serde_json::to_vec_pretty(&adapter).unwrap()
/// # }
/// ```
// All of the data structures in syn-serde are compatible with JSON so unwrap will never fail.
#[allow(clippy::missing_panics_doc)]
pub fn to_vec_pretty<S>(syn: &S) -> Vec<u8>
where
    S: Syn,
{
    let adapter = syn.to_adapter();
    serde_json::to_vec_pretty(&adapter).unwrap()
}

/// Serialize the given [`Syn`] type as a String of JSON.
///
/// This function is equivalent to the following code:
///
/// ```rust
/// # fn to_string(syn_file: &syn::File) -> String {
/// use syn_serde::Syn;
///
/// let adapter = syn_file.to_adapter();
/// serde_json::to_string(&adapter).unwrap()
/// # }
/// ```
// All of the data structures in syn-serde are compatible with JSON so unwrap will never fail.
#[allow(clippy::missing_panics_doc)]
pub fn to_string<S>(syn: &S) -> String
where
    S: Syn,
{
    let adapter = syn.to_adapter();
    serde_json::to_string(&adapter).unwrap()
}

/// Serialize the given [`Syn`] type as a pretty-printed String of JSON.
///
/// This function is equivalent to the following code:
///
/// ```rust
/// # fn to_string_pretty(syn_file: &syn::File) -> String {
/// use syn_serde::Syn;
///
/// let adapter = syn_file.to_adapter();
/// serde_json::to_string_pretty(&adapter).unwrap()
/// # }
/// ```
// All of the data structures in syn-serde are compatible with JSON so unwrap will never fail.
#[allow(clippy::missing_panics_doc)]
pub fn to_string_pretty<S>(syn: &S) -> String
where
    S: Syn,
{
    let adapter = syn.to_adapter();
    serde_json::to_string_pretty(&adapter).unwrap()
}

// Deserialize JSON data to [`Syn`] type.

/// Deserialize an instance of [`Syn`] type from an I/O stream of JSON.
///
/// This function is equivalent to the following code:
///
/// ```rust
/// # use std::io;
/// # fn from_reader<R: io::Read>(reader: R) -> serde_json::Result<syn::File> {
/// use syn_serde::Syn;
///
/// let adapter: <syn::File as Syn>::Adapter = serde_json::from_reader(reader)?;
/// let syn_file = syn::File::from_adapter(&adapter);
/// Ok(syn_file)
/// # }
/// ```
pub fn from_reader<S, R>(reader: R) -> Result<S>
where
    S: Syn,
    R: io::Read,
{
    let adapter: S::Adapter = serde_json::from_reader(reader)?;
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
/// let adapter: <syn::File as Syn>::Adapter = serde_json::from_slice(v)?;
/// let syn_file = syn::File::from_adapter(&adapter);
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
/// let adapter: <syn::File as Syn>::Adapter = serde_json::from_str(s)?;
/// let syn_file = syn::File::from_adapter(&adapter);
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
