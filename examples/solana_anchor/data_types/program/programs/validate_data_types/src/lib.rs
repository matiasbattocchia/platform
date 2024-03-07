// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use std::str::FromStr;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("Djj7zosfYLKGvcGXmxsnDnarLNXxiEPaZ5LP65JohWFS");

#[program]
pub mod validate_data_types {
    use super::*;

/// Test MasterTypeNonPDA types
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[writable, signer]` account: [State] 
/// 2. `[]` account_info_type: [AccountInfo] 
/// 3. `[writable]` account_info_type_mut: [AccountInfo] 
/// 4. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - u8_type: [u8] 
/// - u16_type: [u16] 
/// - u32_type: [u32] 
/// - u64_type: [u64] 
/// - u128_type: [u128] 
/// - i8_type: [i8] 
/// - i16_type: [i16] 
/// - i32_type: [i32] 
/// - i64_type: [i64] 
/// - i128_type: [i128] 
/// - string_type: [String] 
/// - bool_type: [bool] 
/// - f32_type: [f32] 
/// - f64_type: [f64] 
/// - pubkey_type: [Pubkey] 
/// - u8_type_option: [Option<u8>] 
/// - u16_type_option: [Option<u16>] 
/// - u32_type_option: [Option<u32>] 
/// - u64_type_option: [Option<u64>] 
/// - u128_type_option: [Option<u128>] 
/// - i8_type_option: [Option<i8>] 
/// - i16_type_option: [Option<i16>] 
/// - i32_type_option: [Option<i32>] 
/// - i64_type_option: [Option<i64>] 
/// - i128_type_option: [Option<i128>] 
/// - string_type_option: [Option<String>] 
/// - bool_type_option: [Option<bool>] 
/// - f32_type_option: [Option<f32>] 
/// - f64_type_option: [Option<f64>] 
/// - pubkey_type_option: [Option<Pubkey>] 
/// - u8_type_vector: [Vec<u8>] 
/// - u16_type_vector: [Vec<u16>] 
/// - u32_type_vector: [Vec<u32>] 
/// - u64_type_vector: [Vec<u64>] 
/// - u128_type_vector: [Vec<u128>] 
/// - i8_type_vector: [Vec<i8>] 
/// - i16_type_vector: [Vec<i16>] 
/// - i32_type_vector: [Vec<i32>] 
/// - i64_type_vector: [Vec<i64>] 
/// - i128_type_vector: [Vec<i128>] 
/// - bool_type_vector: [Vec<bool>] 
/// - f32_type_vector: [Vec<f32>] 
/// - f64_type_vector: [Vec<f64>] 
/// - pubkey_type_vector: [Vec<Pubkey>] 
	pub fn instruction1(ctx: Context<Instruction1>, u8_type: u8, u16_type: u16, u32_type: u32, u64_type: u64, u128_type: u128, i8_type: i8, i16_type: i16, i32_type: i32, i64_type: i64, i128_type: i128, string_type: String, bool_type: bool, f32_type: f32, f64_type: f64, pubkey_type: Pubkey, u8_type_option: Option<u8>, u16_type_option: Option<u16>, u32_type_option: Option<u32>, u64_type_option: Option<u64>, u128_type_option: Option<u128>, i8_type_option: Option<i8>, i16_type_option: Option<i16>, i32_type_option: Option<i32>, i64_type_option: Option<i64>, i128_type_option: Option<i128>, string_type_option: Option<String>, bool_type_option: Option<bool>, f32_type_option: Option<f32>, f64_type_option: Option<f64>, pubkey_type_option: Option<Pubkey>, u8_type_vector: Vec<u8>, u16_type_vector: Vec<u16>, u32_type_vector: Vec<u32>, u64_type_vector: Vec<u64>, u128_type_vector: Vec<u128>, i8_type_vector: Vec<i8>, i16_type_vector: Vec<i16>, i32_type_vector: Vec<i32>, i64_type_vector: Vec<i64>, i128_type_vector: Vec<i128>, bool_type_vector: Vec<bool>, f32_type_vector: Vec<f32>, f64_type_vector: Vec<f64>, pubkey_type_vector: Vec<Pubkey>) -> Result<()> {
		instruction1::handler(ctx, u8_type, u16_type, u32_type, u64_type, u128_type, i8_type, i16_type, i32_type, i64_type, i128_type, string_type, bool_type, f32_type, f64_type, pubkey_type, u8_type_option, u16_type_option, u32_type_option, u64_type_option, u128_type_option, i8_type_option, i16_type_option, i32_type_option, i64_type_option, i128_type_option, string_type_option, bool_type_option, f32_type_option, f64_type_option, pubkey_type_option, u8_type_vector, u16_type_vector, u32_type_vector, u64_type_vector, u128_type_vector, i8_type_vector, i16_type_vector, i32_type_vector, i64_type_vector, i128_type_vector, bool_type_vector, f32_type_vector, f64_type_vector, pubkey_type_vector)
	}

/// Test MasterTypePDA types
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[writable]` account: [State] 
/// 2. `[]` account_info_type: [AccountInfo] 
/// 3. `[writable]` account_info_type_mut: [AccountInfo] 
/// 4. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - u8_type: [u8] 
/// - u16_type: [u16] 
/// - u32_type: [u32] 
/// - u64_type: [u64] 
/// - u128_type: [u128] 
/// - i8_type: [i8] 
/// - i16_type: [i16] 
/// - i32_type: [i32] 
/// - i64_type: [i64] 
/// - i128_type: [i128] 
/// - string_type: [String] 
/// - bool_type: [bool] 
/// - f32_type: [f32] 
/// - f64_type: [f64] 
/// - pubkey_type: [Pubkey] 
/// - u8_type_option: [Option<u8>] 
/// - u16_type_option: [Option<u16>] 
/// - u32_type_option: [Option<u32>] 
/// - u64_type_option: [Option<u64>] 
/// - u128_type_option: [Option<u128>] 
/// - i8_type_option: [Option<i8>] 
/// - i16_type_option: [Option<i16>] 
/// - i32_type_option: [Option<i32>] 
/// - i64_type_option: [Option<i64>] 
/// - i128_type_option: [Option<i128>] 
/// - string_type_option: [Option<String>] 
/// - bool_type_option: [Option<bool>] 
/// - f32_type_option: [Option<f32>] 
/// - f64_type_option: [Option<f64>] 
/// - pubkey_type_option: [Option<Pubkey>] 
/// - u8_type_vector: [Vec<u8>] 
/// - u16_type_vector: [Vec<u16>] 
/// - u32_type_vector: [Vec<u32>] 
/// - u64_type_vector: [Vec<u64>] 
/// - u128_type_vector: [Vec<u128>] 
/// - i8_type_vector: [Vec<i8>] 
/// - i16_type_vector: [Vec<i16>] 
/// - i32_type_vector: [Vec<i32>] 
/// - i64_type_vector: [Vec<i64>] 
/// - i128_type_vector: [Vec<i128>] 
/// - bool_type_vector: [Vec<bool>] 
/// - f32_type_vector: [Vec<f32>] 
/// - f64_type_vector: [Vec<f64>] 
/// - pubkey_type_vector: [Vec<Pubkey>] 
	pub fn instruction2(ctx: Context<Instruction2>, u8_type: u8, u16_type: u16, u32_type: u32, u64_type: u64, u128_type: u128, i8_type: i8, i16_type: i16, i32_type: i32, i64_type: i64, i128_type: i128, string_type: String, bool_type: bool, f32_type: f32, f64_type: f64, pubkey_type: Pubkey, u8_type_option: Option<u8>, u16_type_option: Option<u16>, u32_type_option: Option<u32>, u64_type_option: Option<u64>, u128_type_option: Option<u128>, i8_type_option: Option<i8>, i16_type_option: Option<i16>, i32_type_option: Option<i32>, i64_type_option: Option<i64>, i128_type_option: Option<i128>, string_type_option: Option<String>, bool_type_option: Option<bool>, f32_type_option: Option<f32>, f64_type_option: Option<f64>, pubkey_type_option: Option<Pubkey>, u8_type_vector: Vec<u8>, u16_type_vector: Vec<u16>, u32_type_vector: Vec<u32>, u64_type_vector: Vec<u64>, u128_type_vector: Vec<u128>, i8_type_vector: Vec<i8>, i16_type_vector: Vec<i16>, i32_type_vector: Vec<i32>, i64_type_vector: Vec<i64>, i128_type_vector: Vec<i128>, bool_type_vector: Vec<bool>, f32_type_vector: Vec<f32>, f64_type_vector: Vec<f64>, pubkey_type_vector: Vec<Pubkey>) -> Result<()> {
		instruction2::handler(ctx, u8_type, u16_type, u32_type, u64_type, u128_type, i8_type, i16_type, i32_type, i64_type, i128_type, string_type, bool_type, f32_type, f64_type, pubkey_type, u8_type_option, u16_type_option, u32_type_option, u64_type_option, u128_type_option, i8_type_option, i16_type_option, i32_type_option, i64_type_option, i128_type_option, string_type_option, bool_type_option, f32_type_option, f64_type_option, pubkey_type_option, u8_type_vector, u16_type_vector, u32_type_vector, u64_type_vector, u128_type_vector, i8_type_vector, i16_type_vector, i32_type_vector, i64_type_vector, i128_type_vector, bool_type_vector, f32_type_vector, f64_type_vector, pubkey_type_vector)
	}


	#[derive(Accounts)]
	#[instruction(
		u8_type: u8,
		u16_type: u16,
		u32_type: u32,
		u64_type: u64,
		u128_type: u128,
		i8_type: i8,
		i16_type: i16,
		i32_type: i32,
		i64_type: i64,
		i128_type: i128,
		string_type: String,
		bool_type: bool,
		f32_type: f32,
		f64_type: f64,
		pubkey_type: Pubkey,
		u8_type_option: Option<u8>,
		u16_type_option: Option<u16>,
		u32_type_option: Option<u32>,
		u64_type_option: Option<u64>,
		u128_type_option: Option<u128>,
		i8_type_option: Option<i8>,
		i16_type_option: Option<i16>,
		i32_type_option: Option<i32>,
		i64_type_option: Option<i64>,
		i128_type_option: Option<i128>,
		string_type_option: Option<String>,
		bool_type_option: Option<bool>,
		f32_type_option: Option<f32>,
		f64_type_option: Option<f64>,
		pubkey_type_option: Option<Pubkey>,
		u8_type_vector: Vec<u8>,
		u16_type_vector: Vec<u16>,
		u32_type_vector: Vec<u32>,
		u64_type_vector: Vec<u64>,
		u128_type_vector: Vec<u128>,
		i8_type_vector: Vec<i8>,
		i16_type_vector: Vec<i16>,
		i32_type_vector: Vec<i32>,
		i64_type_vector: Vec<i64>,
		i128_type_vector: Vec<i128>,
		bool_type_vector: Vec<bool>,
		f32_type_vector: Vec<f32>,
		f64_type_vector: Vec<f64>,
		pubkey_type_vector: Vec<Pubkey>,
	)]
	pub struct Instruction1<'info> {
		#[account(
			mut,
			owner=Pubkey::from_str("11111111111111111111111111111111").unwrap(),
		)]
		pub fee_payer: Signer<'info>,

		#[account(
			signer,
			init,
			space=1391,
			payer=fee_payer,
		)]
		pub account: Account<'info, State>,

		/// CHECK: implement manual checks if needed
		pub account_info_type: UncheckedAccount<'info>,

		#[account(
			mut,
		)]
		/// CHECK: implement manual checks if needed
		pub account_info_type_mut: UncheckedAccount<'info>,

		pub system_program: Program<'info, System>,
	}

	#[derive(Accounts)]
	#[instruction(
		u8_type: u8,
		u16_type: u16,
		u32_type: u32,
		u64_type: u64,
		u128_type: u128,
		i8_type: i8,
		i16_type: i16,
		i32_type: i32,
		i64_type: i64,
		i128_type: i128,
		string_type: String,
		bool_type: bool,
		f32_type: f32,
		f64_type: f64,
		pubkey_type: Pubkey,
		u8_type_option: Option<u8>,
		u16_type_option: Option<u16>,
		u32_type_option: Option<u32>,
		u64_type_option: Option<u64>,
		u128_type_option: Option<u128>,
		i8_type_option: Option<i8>,
		i16_type_option: Option<i16>,
		i32_type_option: Option<i32>,
		i64_type_option: Option<i64>,
		i128_type_option: Option<i128>,
		string_type_option: Option<String>,
		bool_type_option: Option<bool>,
		f32_type_option: Option<f32>,
		f64_type_option: Option<f64>,
		pubkey_type_option: Option<Pubkey>,
		u8_type_vector: Vec<u8>,
		u16_type_vector: Vec<u16>,
		u32_type_vector: Vec<u32>,
		u64_type_vector: Vec<u64>,
		u128_type_vector: Vec<u128>,
		i8_type_vector: Vec<i8>,
		i16_type_vector: Vec<i16>,
		i32_type_vector: Vec<i32>,
		i64_type_vector: Vec<i64>,
		i128_type_vector: Vec<i128>,
		bool_type_vector: Vec<bool>,
		f32_type_vector: Vec<f32>,
		f64_type_vector: Vec<f64>,
		pubkey_type_vector: Vec<Pubkey>,
	)]
	pub struct Instruction2<'info> {
		#[account(
			mut,
			owner=Pubkey::from_str("11111111111111111111111111111111").unwrap(),
		)]
		pub fee_payer: Signer<'info>,

		#[account(
			init,
			space=1391,
			payer=fee_payer,
			seeds = [
				b"master_type_pda",
			],
			bump,
		)]
		pub account: Account<'info, State>,

		/// CHECK: implement manual checks if needed
		pub account_info_type: UncheckedAccount<'info>,

		#[account(
			mut,
		)]
		/// CHECK: implement manual checks if needed
		pub account_info_type_mut: UncheckedAccount<'info>,

		pub system_program: Program<'info, System>,
	}
}
