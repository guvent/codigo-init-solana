// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use crate::generated::errors::CounterError;

#[derive(BorshSerialize, Debug)]
pub enum CounterInstruction {
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[]` greeting_account: [GreetingAccount] 
	Increment,

/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[]` greeting_account: [GreetingAccount] 
	Decrement,

}

impl CounterInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input.split_first().ok_or(CounterError::InvalidInstruction)?;

        Ok(match variant {
			0 => Self::Increment,
			1 => Self::Decrement,
			_ => return Err(CounterError::InvalidInstruction.into())
        })
    }
}