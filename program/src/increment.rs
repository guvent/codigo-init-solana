use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::state::{
	Account,
	GreetingAccount,
};


/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[]` greeting_account: [GreetingAccount] 
pub fn increment(
	program_id: &Pubkey,
	greeting_account: &Account<GreetingAccount>,
) -> ProgramResult {
    // Implement your business logic here...






    Ok(())
}