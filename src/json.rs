//! A module to provide functions for JSON <-> Rust serialize and deserialize.
//!
//! *This module is available if Serde Syn is built with the `"json"` feature.*

use super::*;
use serde_json::Result;
use std::io;

// Serialize `syn::File` into JSON data.

/// Serialize the given [`syn::File`] as JSON into the IO stream.
///
/// This function is equivalent to the following code:
///
/// ```rust
/// # use std::io;
/// # fn to_writer<W>(writer: W, syn_file: &syn::File) -> serde_json::Result<()>
/// # where
/// #     W: io::Write,
/// # {
/// use serde_syn::File;
///
/// let serializable_file = File::from(syn_file);
/// serde_json::to_writer(writer, &serializable_file)
/// # }
/// ```
///
/// [`syn::File`]: https://docs.rs/syn/0.15/syn/struct.File.html
#[inline]
pub fn to_writer<W>(writer: W, syn_file: &syn::File) -> Result<()>
where
    W: io::Write,
{
    let syntax = File::from(syn_file);
    serde_json::to_writer(writer, &syntax)
}

/// Serialize the given [`syn::File`] as pretty-printed JSON into the IO
/// stream.
///
/// This function is equivalent to the following code:
///
/// ```rust
/// # use std::io;
/// # fn to_writer_pretty<W>(writer: W, syn_file: &syn::File) -> serde_json::Result<()>
/// # where
/// #     W: io::Write,
/// # {
/// use serde_syn::File;
///
/// let serializable_file = File::from(syn_file);
/// serde_json::to_writer_pretty(writer, &serializable_file)
/// # }
/// ```
///
/// [`syn::File`]: https://docs.rs/syn/0.15/syn/struct.File.html
#[inline]
pub fn to_writer_pretty<W>(writer: W, syn_file: &syn::File) -> Result<()>
where
    W: io::Write,
{
    let syntax = File::from(syn_file);
    serde_json::to_writer_pretty(writer, &syntax)
}

/// Serialize the given [`syn::File`] as a JSON byte vector.
///
/// This function is equivalent to the following code:
///
/// ```rust
/// # fn to_vec(syn_file: &syn::File) -> serde_json::Result<Vec<u8>> {
/// use serde_syn::File;
///
/// let serializable_file = File::from(syn_file);
/// serde_json::to_vec(&serializable_file)
/// # }
/// ```
///
/// [`syn::File`]: https://docs.rs/syn/0.15/syn/struct.File.html
#[inline]
pub fn to_vec(syn_file: &syn::File) -> Result<Vec<u8>> {
    let syntax = File::from(syn_file);
    serde_json::to_vec(&syntax)
}

/// Serialize the given [`syn::File`] as a pretty-printed JSON byte vector.
///
/// This function is equivalent to the following code:
///
/// ```rust
/// # fn to_vec_pretty(syn_file: &syn::File) -> serde_json::Result<Vec<u8>> {
/// use serde_syn::File;
///
/// let serializable_file = File::from(syn_file);
/// serde_json::to_vec_pretty(&serializable_file)
/// # }
/// ```
///
/// [`syn::File`]: https://docs.rs/syn/0.15/syn/struct.File.html
#[inline]
pub fn to_vec_pretty(syn_file: &syn::File) -> Result<Vec<u8>> {
    let syntax = File::from(syn_file);
    serde_json::to_vec_pretty(&syntax)
}

/// Serialize the given [`syn::File`] as a String of JSON.
///
/// This function is equivalent to the following code:
///
/// ```rust
/// # fn to_string(syn_file: &syn::File) -> serde_json::Result<String> {
/// use serde_syn::File;
///
/// let serializable_file = File::from(syn_file);
/// serde_json::to_string(&serializable_file)
/// # }
/// ```
///
/// [`syn::File`]: https://docs.rs/syn/0.15/syn/struct.File.html
#[inline]
pub fn to_string(syn_file: &syn::File) -> Result<String> {
    let syntax = File::from(syn_file);
    serde_json::to_string(&syntax)
}

/// Serialize the given [`syn::File`] as a pretty-printed String of JSON.
///
/// This function is equivalent to the following code:
///
/// ```rust
/// # fn to_string_pretty(syn_file: &syn::File) -> serde_json::Result<String> {
/// use serde_syn::File;
///
/// let serializable_file = File::from(syn_file);
/// serde_json::to_string_pretty(&serializable_file)
/// # }
/// ```
///
/// [`syn::File`]: https://docs.rs/syn/0.15/syn/struct.File.html
#[inline]
pub fn to_string_pretty(syn_file: &syn::File) -> Result<String> {
    let syntax = File::from(syn_file);
    serde_json::to_string_pretty(&syntax)
}

// Deserialize JSON data to `syn::File`.

/// Deserialize an instance of [`syn::File`] from an IO stream of JSON.
///
/// This function is equivalent to the following code:
///
/// ```rust
/// # use std::io;
/// # fn from_reader<R>(rdr: R) -> serde_json::Result<syn::File>
/// # where
/// #     R: io::Read,
/// # {
/// use serde_syn::File;
///
/// let serializable_file: File = serde_json::from_reader(rdr)?;
/// let syn_file = syn::File::from(&serializable_file);
/// Ok(syn_file)
/// # }
/// ```
///
/// [`syn::File`]: https://docs.rs/syn/0.15/syn/struct.File.html
pub fn from_reader<R>(rdr: R) -> Result<syn::File>
where
    R: io::Read,
{
    let syntax: File = serde_json::from_reader(rdr)?;
    Ok(syntax.ref_into())
}

/// Deserialize an instance of [`syn::File`] from bytes of JSON text.
///
/// This function is equivalent to the following code:
///
/// ```rust
/// # fn from_reader(v: &[u8]) -> serde_json::Result<syn::File> {
/// use serde_syn::File;
///
/// let serializable_file: File = serde_json::from_slice(v)?;
/// let syn_file = syn::File::from(&serializable_file);
/// Ok(syn_file)
/// # }
/// ```
///
/// [`syn::File`]: https://docs.rs/syn/0.15/syn/struct.File.html
pub fn from_slice(v: &[u8]) -> Result<syn::File> {
    let syntax: File = serde_json::from_slice(v)?;
    Ok(syntax.ref_into())
}

/// Deserialize an instance of [`syn::File`] from a string of JSON text.
///
/// This function is equivalent to the following code:
///
/// ```rust
/// # fn from_str(s: &str) -> serde_json::Result<syn::File> {
/// use serde_syn::File;
///
/// let serializable_file: File = serde_json::from_str(s)?;
/// let syn_file = syn::File::from(&serializable_file);
/// Ok(syn_file)
/// # }
/// ```
///
/// [`syn::File`]: https://docs.rs/syn/0.15/syn/struct.File.html
pub fn from_str(s: &str) -> Result<syn::File> {
    let syntax: File = serde_json::from_str(s)?;
    Ok(syntax.ref_into())
}
