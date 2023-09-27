use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;



/// Reference `validate_advanced_cases.instruction28`, where caller doesn't have inputs
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[signer]` signer_1: [AccountInfo] 
/// 2. `[signer]` signer_2: [AccountInfo] 
/// 3. `[writable, signer]` signer_3: [AccountInfo] 
/// 4. `[writable]` account: [AccountInfo] 
/// 5. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
/// 6. `[]` validate_advanced_cases_v_0_0_0: [AccountInfo] Auto-generated, ValidateAdvancedCasesProgram v0.0.0
pub fn instruction_48(
	program_id: &Pubkey,
	for_instruction_28: &[&AccountInfo],
	signer_1: &AccountInfo,
	account: &AccountInfo,
) -> ProgramResult {
    // Implement your business logic here...

	validate_advanced_cases::src::cpi::instruction_28(
		for_instruction_28,
		48,
		8,
		16,
		32,
		8,
		16,
		32,
		String::from("ix48"),
		*signer_1.key,
	)?;

    Ok(())
}