use crate::CRATE_DECKTRICKS_LOGGER;
use std::sync::LazyLock;
use std::sync::Arc;

use decktricks::prelude::*;

// For use only within this crate, and not within logging.rs:
static EARLY_LOGGING_CONTEXT: LazyLock<Arc<GeneralExecutionContext>> =
    LazyLock::new(|| {
        Arc::new(GeneralExecutionContext::internal_for_gui_startup(
            get_log_level(),
            CRATE_DECKTRICKS_LOGGER.clone(),
        ))
    });

// For use only within this crate, and not within logging.rs:
pub(crate) fn early_log_ctx() -> &'static GeneralExecutionContext {
    &EARLY_LOGGING_CONTEXT
}
