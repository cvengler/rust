//! Utilities for the `Option` core enum.

use crate::borrow::ToOwned;

/// Implements `Option::map_owned()`.
///
/// It may seem odd why we implement `ToOwned` for `&T` instead of `T`, but
/// this is a necessity.
///
/// Assume one would implement this for `Option<T>` instead, then the
/// auto-referencing mechanism would transform `x.to_owned()` to
/// `ToOwned::to_owned(&x)`, thereby creating a double reference, which would
/// not lead to the desired result, as owning a `&&str` would result in a `&str`
/// and not the desired `String`.
impl<T: ?Sized + ToOwned> Option<&T> {
    /// Maps an `Option<T>` to `Option<T::Owned>` by calling [`ToOwned::to_owned()`].
    ///
    /// # Examples
    ///
    /// Calls a method from [`str`] on a [`String`](crate::string::String) while
    /// staying a [`String`](crate::string::String) in the returned [`Option`].
    /// ```
    /// #![feature(option_map_owned)]
    /// let input: String = String::from("foobar");
    /// let stripped: Option<String> = input.strip_prefix("foo").map_owned();
    /// assert_eq!(stripped, Some("bar".to_string()));
    /// ```
    #[unstable(feature = "option_map_owned", issue = "none")]
    #[rustc_allow_incoherent_impl]
    pub fn map_owned(self) -> Option<T::Owned> {
        self.map(|x| x.to_owned())
    }
}
