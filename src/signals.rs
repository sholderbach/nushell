use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use ctrlc;

use nu_protocol::engine::EngineState;

/// Wires up a signal handler to detect `SIGINT`/`CtrlC`
///
/// Warning: Should only be called once for the process
/// https://docs.rs/ctrlc/latest/ctrlc/fn.set_handler.html
///
/// This has the consequence that the main `EngineState` should be kept alive
/// (if you extract a `CancelFlag` this could be used to restore to same provenance)
pub(crate) fn ctrlc_protection(engine_state: &mut EngineState) -> Arc<AtomicBool> {
    let handler_ctrlc = Arc::new(AtomicBool::new(false));
    let engine_state_ctrlc = handler_ctrlc.clone();

    ctrlc::set_handler(move || {
        handler_ctrlc.store(true, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // TODO remove return value as soon as we migrate to `CancelFlag`
    let ctrlc = engine_state_ctrlc.clone();

    engine_state.initialize_cancel_flag(engine_state_ctrlc);

    ctrlc
}
