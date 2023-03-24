use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::{Pack, IsInitialized},
    sysvar::{rent::Rent, Sysvar},
    instruction::{AccountMeta, Instruction},
    program_pack::Pack,
};
use spl_token::{
    self,
    state::{Account, Mint},
    instruction::{initialize_mint, initialize_account, mint_to},
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = instruction_data
        .get(..4)
        .and_then(|slice| slice.try_into().ok())
        .map(u32::from_le_bytes)
        .ok_or_else(|| ProgramError::InvalidInstructionData)?;
    
    match instruction {
        0 => {
            // Initialize the mint account
            msg!("Instruction: Initialize mint");
            let accounts_iter = &mut accounts.iter();
            let mint_account = next_account_info(accounts_iter)?;
            let mint_authority = next_account_info(accounts_iter)?;
            let rent_sysvar_info = next_account_info(accounts_iter)?;

            let rent = &Rent::from_account_info(rent_sysvar_info)?;
            let rent_exempt_balance = rent.minimum_balance(Mint::LEN);

            let mut mint = Mint::unpack_unchecked(&mint_account.data.borrow())?;
            if mint.is_initialized() {
                return Err(ProgramError::AccountAlreadyInitialized);
            }
            mint.mint_authority = *mint_authority.key;
            mint.freeze_authority = None;
            Mint::pack(mint, &mut mint_account.data.borrow_mut())?;

            spl_token::native_mint::initialize_account(
                &spl_token::id(),
                mint_account.key,
                &spl_token::native_mint::id(),
                mint_authority.key,
            )?;

            Ok(())
        }
        1 => {
            // Initialize the token account
            msg!("Instruction: Initialize account");
            let accounts_iter = &mut accounts.iter();
            let token_account = next_account_info(accounts_iter)?;
            let mint_account = next_account_info(accounts_iter)?;
            let owner_account = next_account_info(accounts_iter)?;
            let rent_sysvar_info = next_account_info(accounts_iter)?;

            let rent = &Rent::from_account_info(rent_sysvar_info)?;
            let rent_exempt_balance = rent.minimum_balance(Account::LEN);

            let mut account = Account::unpack_unchecked(&token_account.data.borrow())?;
            if account.is_initialized() {
                return Err(ProgramError::AccountAlreadyInitialized);
            }

            account.mint = *mint_account.key;
            account.owner = *owner_account.key;
            account.amount = 0;
            account.state = spl_token::state::AccountState::Initialized;
            Account::pack(account, &mut token_account.data.borrow_mut())?;

            let account_metas = vec![
                AccountMeta::new(*token_account.key, false),
                AccountMeta::new_readonly(*owner_account.key, true),
                AccountMeta::new_readonly(*mint_account.key, false),
                AccountMeta::new_readonly(spl_token::id(), false),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(sysvar::rent::id(), false),
            ];
            let initialize_account_instruction = initialize_account(
                &spl_token::id(),
                token_account.key,
                mint_account.key,
                owner_account.key,
            );

            let
