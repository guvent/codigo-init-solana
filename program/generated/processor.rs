// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use std::str::FromStr;
use borsh::BorshSerialize;
use solana_program::account_info::{AccountInfo, next_account_info, next_account_infos};
use solana_program::borsh0_10::try_from_slice_unchecked;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction::create_account;
use solana_program::{msg, system_program};
use solana_program::sysvar::Sysvar;
use solana_program::program_pack::Pack;
use crate::generated::errors::CounterError;
use crate::generated::instructions::CounterInstruction;

use crate::generated::state::{
	Account,
	AccountPDA,
	GreetingAccount,
};
use crate::src::*;

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: &[u8],
    ) -> ProgramResult {
        let instruction = CounterInstruction::unpack(data)?;

        match instruction {
			CounterInstruction::Increment => {
				msg!("Instruction: Increment");
				Self::process_increment(program_id, accounts)
			}
			CounterInstruction::Decrement => {
				msg!("Instruction: Decrement");
				Self::process_decrement(program_id, accounts)
			}
        }
    }

/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[]` greeting_account: [GreetingAccount] 
	pub fn process_increment(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let greeting_account_info = next_account_info(account_info_iter)?;


		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(CounterError::InvalidSignerPermission.into());
		}



		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(CounterError::WrongAccountOwner.into());
		}

		if *greeting_account_info.owner != *program_id {
			return Err(CounterError::WrongAccountOwner.into());
		}

		if greeting_account_info.data_len() != GreetingAccount::LEN {
			return Err(CounterError::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let greeting_account = Account::new(
			&greeting_account_info,
			try_from_slice_unchecked::<GreetingAccount>(&greeting_account_info.data.borrow()).unwrap(),
		);

		// Calling STUB
		increment::increment(
			program_id,
			&greeting_account,
		)?;
		
		Ok(())
	}

/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[]` greeting_account: [GreetingAccount] 
	pub fn process_decrement(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let greeting_account_info = next_account_info(account_info_iter)?;


		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(CounterError::InvalidSignerPermission.into());
		}



		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(CounterError::WrongAccountOwner.into());
		}

		if *greeting_account_info.owner != *program_id {
			return Err(CounterError::WrongAccountOwner.into());
		}

		if greeting_account_info.data_len() != GreetingAccount::LEN {
			return Err(CounterError::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let greeting_account = Account::new(
			&greeting_account_info,
			try_from_slice_unchecked::<GreetingAccount>(&greeting_account_info.data.borrow()).unwrap(),
		);

		// Calling STUB
		decrement::decrement(
			program_id,
			&greeting_account,
		)?;
		
		Ok(())
	}
}