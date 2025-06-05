//! A collection of traits with more semantic method names.
//!
//! This crate provides traits which wrap existing methods, only renaming
//! the method calls to make them more distinct.
//!
//! There are trait implementations available under feature flags.
//! `"std"` is enabled by default. If there are other crates/types
//! that you think should implement one of these traits, feel free
//! to open an issue/PR.
//!
//! You can also implement these traits for your own types.
//!
//! # Examples
//!
//! ## Shared References
//!
//! The [`Share`] trait is an alias for [`Clone`].
//! It should be implemented on types that point to the same underlying
//! data when cloned (like [`Rc`]/[`Arc`]), or more generally share
//! some kind of state when cloned (like [`mpsc`] channel senders).
//!
//! You may want to enable the `clippy::clone_on_ref_ptr` lint for
//! a warning when you clone on an [`Rc`]/[`Arc`].
//!
//! ```
//! use std::sync::{Arc, mpsc};
//! use std::thread;
//! use semtraits::Share;
//!
//! let foo = Arc::new(vec![1, 2, 3]);
//! // The two lines below are equivalent.
//! let bar = foo.share();
//! let baz = Arc::share(&foo);
//! assert!(Arc::ptr_eq(&bar, &baz));
//!
//! // An mpsc sender can also 'share' it's sender.
//! let (tx, rx) = mpsc::channel();
//! let tx1 = tx.share();
//! thread::spawn(move || {
//!     tx1.send(10).unwrap();
//! });
//! thread::spawn(move || {
//!     tx.send(10).unwrap();
//! });
//! assert_eq!(rx.recv().unwrap(), 10);
//! assert_eq!(rx.recv().unwrap(), 10);
//! ```
//!
//! ## Disconnected Channels
//!
//! The [`OrClosed`] trait is an alias for panicking when a channel
//! is closed but one side tries to send/receive.
//!
//! The above example of the [`Share`] trait on channels highlights
//! when [`OrClosed`] may be used.
//!
//! ```
//! use std::sync::mpsc;
//! use std::thread;
//! use semtraits::{Share, OrClosed};
//!
//! let (tx, rx) = mpsc::channel();
//! let tx1 = tx.share();
//! thread::spawn(move || {
//!     // same as .expect(...)
//!     tx1.send(10).or_closed();
//! });
//! thread::spawn(move || {
//!     tx.send(10).or_closed();
//! });
//! assert_eq!(rx.recv().or_closed(), 10);
//! assert_eq!(rx.recv().or_closed(), 10);
//! ```
//!
//! This trait is implemented on the output of `tx.send()`
//! and `rx.recv()`. It should be implemented on channel send/recv
//! methods when an error means that it is impossible for the
//! channel to send/receive a value again.
//!
//! ## Poisoned Locks
//!
//! The [`OrPoisoned`] trait is an alias for panicking when a lock has
//! been poisoned.
//!
//! ```
//! use std::sync::Mutex;
//! use semtraits::OrPoisoned;
//!
//! let mutex = Mutex::new(1);
//!
//! {
//!     let mut data = mutex.lock().or_poisoned();
//!     *data = 2;
//! }
//!
//! *mutex.lock().or_poisoned() += 10;
//!
//! assert_eq!(*mutex.lock().or_poisoned(), 12);
//! ```
//!
//! [`Rc`]: std::rc::Rc
//! [`Arc`]: std::sync::Arc
//! [`mpsc`]: std::sync::mpsc

#![no_std]

#[cfg(feature = "std")]
extern crate std;

mod impls;

/// A cheap [`Clone`] with shared reference semantics.
///
/// This is implemented on types that point to the same underlying
/// data when cloned (like [`Rc`]/[`Arc`]), or more generally share
/// some kind of state when cloned (like [`mpsc`] channel senders).
///
/// You may want to enable the `clippy::clone_on_ref_ptr` lint for
/// a warning when you clone on an [`Rc`]/[`Arc`].
///
/// ```
/// use std::sync::{Arc, mpsc};
/// use std::thread;
/// use semtraits::Share;
///
/// let foo = Arc::new(vec![1, 2, 3]);
/// // The two lines below are equivalent.
/// let bar = foo.share();
/// let baz = Arc::share(&foo);
/// assert!(Arc::ptr_eq(&bar, &baz));
///
/// // An mpsc sender can also 'share' it's sender.
/// let (tx, rx) = mpsc::channel();
/// let tx1 = tx.share();
/// thread::spawn(move || {
///     tx1.send(10).unwrap();
/// });
/// thread::spawn(move || {
///     tx.send(10).unwrap();
/// });
/// assert_eq!(rx.recv().unwrap(), 10);
/// assert_eq!(rx.recv().unwrap(), 10);
/// ```
///
/// [`Rc`]: std::rc::Rc
/// [`Arc`]: std::sync::Arc
/// [`mpsc`]: std::sync::mpsc
pub trait Share: Clone {
    fn share(&self) -> Self {
        self.clone()
    }
}

/// Panic when a channel is closed but one side tries to send/receive.
///
/// This should be implemented on channels where an error is returned
/// by `send`/`recv` methods *if and only if* it is impossible for
/// the channel to send/receive a value again (i.e. the channel is
/// disconnected/closed).
///
/// ```
/// use std::sync::mpsc;
/// use semtraits::OrClosed;
///
/// let (tx, rx) = mpsc::channel();
/// // same as .expect(...)
/// tx.send(10).or_closed();
/// assert_eq!(rx.recv().or_closed(), 10);
/// ```
///
/// This trait is implemented on the output of `tx.send()` and
/// `rx.recv()` methods, which are generally [`Result`] types with
/// a channel-specific error type.
pub trait OrClosed {
    type Value;

    fn or_closed(self) -> Self::Value;
}

/// Gets the value out of a lock, panic if the lock has been poisoned.
///
/// ```
/// use std::sync::Mutex;
/// use semtraits::OrPoisoned;
///
/// let mutex = Mutex::new(1);
///
/// {
///     let mut data = mutex.lock().or_poisoned();
///     *data = 2;
/// }
///
/// *mutex.lock().or_poisoned() += 10;
///
/// assert_eq!(*mutex.lock().or_poisoned(), 12);
/// ```
pub trait OrPoisoned {
    type Value;

    fn or_poisoned(self) -> Self::Value;
}
