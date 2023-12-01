//! Example Rust-based BPF program that panics

#[cfg(all(feature = "custom-panic", target_arch = "bpf"))]
#[no_mangle]
fn custom_panic(info: &core::panic::PanicInfo<'_>) {
    // Note: Full panic reporting is included here for testing purposes
    nexis_program::msg!("program custom panic enabled");
    nexis_program::msg!(&format!("{}", info));
}

extern crate nexis_program;
use nexis_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

entrypoint!(process_instruction);
#[allow(clippy::unnecessary_wraps)]
fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    assert_eq!(1, 2);
    Ok(())
}
