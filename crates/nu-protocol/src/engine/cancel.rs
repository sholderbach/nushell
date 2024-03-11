use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Flag to check if execution has been cancelled
///
/// Currently this only takes care of `Ctrl-C` but could be extended both to other signals or
/// situations where aborting should be triggered.
///
/// By using this abstraction we can guarantee consistent basic behavior but could easily expand to
/// more signals/conditions.
#[derive(Debug, Clone)]
pub struct CancelFlag {
    ctrlc: Arc<AtomicBool>,
}

impl CancelFlag {
    /// `Ctrl-C` has been pressed or `SIGINT` been received.
    ///
    /// General nushell code should stop what it is doing, perform any special cleanup, or return
    /// the necessary errors.
    #[inline]
    pub fn is_interrupting(&self) -> bool {
        // TODO: figure out if `Ordering::Relaxed` is fine
        self.ctrlc.load(Ordering::SeqCst)
    }

    /// Only to be set up when initializing the [`EngineState`](crate::EngineState)
    ///
    /// Further instances are to be created by cloning
    pub(super) fn init(ctrlc: Arc<AtomicBool>) -> Self {
        CancelFlag { ctrlc }
    }

    /// Placeholder when initializing the [`EngineState`](crate::EngineState)
    ///
    /// Should be replaced with a working version with [`CancelFlag::init`]
    ///
    /// This type actively discourages any `Default` implementation
    pub(super) fn placeholder() -> Self {
        CancelFlag { ctrlc: Arc::new(AtomicBool::new(false)) }
    }

    pub fn into_raw(self) -> Arc<AtomicBool> {
        self.ctrlc
    }
}
