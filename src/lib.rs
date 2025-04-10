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
//! It should be used for clones that semantically represent some kind
//! of reference counting or generally refer to the same object.
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
//! The [`OrHung`] trait is an alias for panicking when a channel
//! sender or receiver is disconnected/hung up.
//!
//! The above example of the [`Share`] trait on channels highlights
//! when [`OrHung`] may be used.
//!
//! ```
//! use std::sync::mpsc;
//! use std::thread;
//! use semtraits::{Share, OrHung};
//!
//! let (tx, rx) = mpsc::channel();
//! let tx1 = tx.share();
//! thread::spawn(move || {
//!     // same as .expect(...)
//!     tx1.send(10).or_hung();
//! });
//! thread::spawn(move || {
//!     tx.send(10).or_hung();
//! });
//! assert_eq!(rx.recv().or_hung(), 10);
//! assert_eq!(rx.recv().or_hung(), 10);
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
//! use semtraits::{Share, OrPoisoned};
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
//! [`Clone`]: std::clone::Clone
//! [`Rc`]: std::rc::Rc
//! [`Arc`]: std::sync::Arc

#![no_std]

#[cfg(feature = "std")]
extern crate std;

mod impls;

pub trait Share: Clone {
    fn share(&self) -> Self {
        self.clone()
    }
}

pub trait OrHung {
    type Value;

    fn or_hung(self) -> Self::Value;
}

pub trait OrPoisoned {
    type Value;

    fn or_poisoned(self) -> Self::Value;
}
