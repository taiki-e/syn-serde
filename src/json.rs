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
pub fn to_writer<S, W>(writer: W, syn: &S) -> Result<()>
where
    S: Syn,
    W: io::Write,
{
    let adapter = syn.to_adapter();
    serde_json::to_writer(writer, &adapter)
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
pub fn to_writer_pretty<S, W>(writer: W, syn: &S) -> Result<()>
where
    S: Syn,
    W: io::Write,
{
    let adapter = syn.to_adapter();
    serde_json::to_writer_pretty(writer, &adapter)
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
pub fn to_vec<S>(syn: &S) -> Result<Vec<u8>>
where
    S: Syn,
{
    let adapter = syn.to_adapter();
    serde_json::to_vec(&adapter)
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
pub fn to_vec_pretty<S>(syn: &S) -> Result<Vec<u8>>
where
    S: Syn,
{
    let adapter = syn.to_adapter();
    serde_json::to_vec_pretty(&adapter)
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
pub fn to_string<S>(syn: &S) -> Result<String>
where
    S: Syn,
{
    let adapter = syn.to_adapter();
    serde_json::to_string(&adapter)
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
pub fn to_string_pretty<S>(syn: &S) -> Result<String>
where
    S: Syn,
{
    let adapter = syn.to_adapter();
    serde_json::to_string_pretty(&adapter)
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
pub fn from_reader<S, R>(rdr: R) -> Result<S>
where
    S: Syn,
    R: io::Read,
{
    let adapter: S::Adapter = serde_json::from_reader(rdr)?;
    Ok(S::from_adapter(&adapter))
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
pub fn from_slice<S>(v: &[u8]) -> Result<S>
where
    S: Syn,
{
    let adapter: S::Adapter = serde_json::from_slice(v)?;
    Ok(S::from_adapter(&adapter))
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
pub fn from_str<S>(s: &str) -> Result<S>
where
    S: Syn,
{
    let adapter: S::Adapter = serde_json::from_str(s)?;
    Ok(S::from_adapter(&adapter))
}
