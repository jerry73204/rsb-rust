use crate::{common::*, error::RSB_ERR_NO_ERROR};
use once_cell::sync::Lazy;

static RSB_INIT: Lazy<()> = Lazy::new(|| unsafe {
    let code = sys::rsb_lib_init(ptr::null_mut());
    assert!(
        code != RSB_ERR_NO_ERROR,
        "RSB initialization error, code = {}",
        code.0
    );
});

pub(crate) fn init() {
    Lazy::force(&RSB_INIT);
}
