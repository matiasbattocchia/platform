use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;



/// Reference `validate_accounts.instruction14`, where caller doesn't have inputs
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[writable]` account: [AccountInfo] 
/// 2. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
/// 3. `[]` validate_accounts_v_0_0_0: [AccountInfo] Auto-generated, ValidateAccountsProgram v0.0.0
pub fn instruction_14(
	program_id: &Pubkey,
	for_instruction_14: &[&AccountInfo],
	account: &AccountInfo,
) -> ProgramResult {
    // Implement your business logic here...

	validate_accounts::src::cpi::instruction_14(
		for_instruction_14,
		14,
	)?;

    Ok(())
}