use crate::host::host_log;

#[no_mangle]
pub fn host_log3() {
    host_log::<3>();
}
