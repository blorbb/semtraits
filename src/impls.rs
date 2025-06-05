//! Trait implementations on std and other common crates.

const SEND_PANIC_MESSAGE: &str = "sending with disconnected receiver";
const RECV_PANIC_MESSAGE: &str = "receiving with no senders";
const POISON_PANIC_MESSAGE: &str = "lock poisoned";

#[cfg(feature = "std")]
mod std {
    use std::{
        rc::{self, Rc},
        sync::{
            self, Arc, LockResult,
            mpsc::{RecvError, SendError, Sender, SyncSender},
        },
    };

    use super::{POISON_PANIC_MESSAGE, RECV_PANIC_MESSAGE, SEND_PANIC_MESSAGE};
    use crate::{OrClosed, OrPoisoned, Share};

    impl<T> Share for Rc<T> {}
    impl<T> Share for Arc<T> {}
    impl<T> Share for rc::Weak<T> {}
    impl<T> Share for sync::Weak<T> {}
    impl<T> Share for Sender<T> {}
    impl<T> Share for SyncSender<T> {}

    impl<T, E> OrClosed for Result<T, SendError<E>> {
        type Value = T;

        fn or_closed(self) -> Self::Value {
            self.expect(SEND_PANIC_MESSAGE)
        }
    }

    impl<T> OrClosed for Result<T, RecvError> {
        type Value = T;

        fn or_closed(self) -> Self::Value {
            self.expect(RECV_PANIC_MESSAGE)
        }
    }

    impl<T> OrPoisoned for LockResult<T> {
        type Value = T;

        fn or_poisoned(self) -> T {
            self.expect(POISON_PANIC_MESSAGE)
        }
    }
}

#[cfg(feature = "tokio")]
mod tokio {
    use tokio::sync::{mpsc, oneshot, watch};

    use super::{RECV_PANIC_MESSAGE, SEND_PANIC_MESSAGE};
    use crate::{OrClosed, Share};

    impl<T> Share for mpsc::Sender<T> {}
    impl<T> Share for mpsc::UnboundedSender<T> {}
    impl<T> Share for mpsc::WeakSender<T> {}
    impl<T> Share for mpsc::WeakUnboundedSender<T> {}
    impl<T> Share for watch::Sender<T> {}
    impl<T> Share for watch::Receiver<T> {}

    impl<T, E> OrClosed for Result<T, mpsc::error::SendError<E>> {
        type Value = T;

        fn or_closed(self) -> T {
            self.expect(SEND_PANIC_MESSAGE)
        }
    }

    // mpsc recv returns an Option instead of Result :(

    impl<T, E> OrClosed for Result<T, watch::error::SendError<E>> {
        type Value = T;

        fn or_closed(self) -> T {
            self.expect(SEND_PANIC_MESSAGE)
        }
    }

    impl<T> OrClosed for Result<T, watch::error::RecvError> {
        type Value = T;

        fn or_closed(self) -> Self::Value {
            self.expect(RECV_PANIC_MESSAGE)
        }
    }

    // oneshot send returns a Result<(), T> :(

    impl<T> OrClosed for Result<T, oneshot::error::RecvError> {
        type Value = T;

        fn or_closed(self) -> Self::Value {
            self.expect(RECV_PANIC_MESSAGE)
        }
    }

    // should not be implemented for broadcast channels.
    // recv error is Closed or Lagged, which should usually be handled manually.
    // sender could be subscribed to after trying to send to no receivers,
    // so it can return Ok after an Err.
}
