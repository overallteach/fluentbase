use crate::common_sp::{stack_push_u256, u256_zero, SP_BASE_MEM_OFFSET_DEFAULT};

// #[no_mangle]
// pub fn host_env_blobbasefee() {
//     // let v: [u8; 8] = ExecutionContext::tx_blob_gas_price().to_be_bytes();
//
//     // stack_push_u256(SP_BASE_MEM_OFFSET_DEFAULT, u256_from_be_slice(&v));
//     stack_push_u256(SP_BASE_MEM_OFFSET_DEFAULT, u256_zero());
// }
