use solana_client::{rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_instruction,
    system_program, 
    program_pack::Pack,
};
use spl_token::{
    self, 
    instruction::{initialize_account, initialize_mint, mint_to},
    state::{Account as TokenAccount, Mint},
};

fn main() {
    // Set the network and token account addresses
    let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
    let token_account_address = Pubkey::from_str("<TOKEN_ACCOUNT_ADDRESS>").unwrap();

    // Set the payer account and its associated token account
    let payer_account = solana_sdk::signature::Keypair::from_bytes(&hex::decode("<PAYER_PRIVATE_KEY>").unwrap()).unwrap();
    let associated_token_address = spl_token::state::Account::get_associated_address(
        &payer_account.pubkey(),
        &spl_token::id(),
        &Pubkey::from_str("<TOKEN_MINT_ADDRESS>").unwrap(),
    );

    // Connect to the Solana RPC endpoint
    let rpc_client = RpcClient::new(rpc_url);

    // Create the token object
    let token_program_id = spl_token::id();
    let token = Token::new(&rpc_client, Pubkey::from_str("<TOKEN_MINT_ADDRESS>").unwrap());

    // Create the transaction instructions
    let mut instructions = vec![];

    // Create the token account if it doesn't exist
    let token_account_exists = match rpc_client.get_account(&token_account_address) {
        Ok(Some(_)) => true,
        _ => false,
    };
    if !token_account_exists {
        let create_account_instruction = system_instruction::create_account(
            &payer_account.pubkey(),
            &token_account_address,
            1.max(token_account_layout.span()),
            token_account_layout.span(),
            &token_program_id,
        );
        instructions.push(create_account_instruction);
    }

    // Create the associated token account if it doesn't exist
    let associated_token_account_exists = match rpc_client.get_account(&associated_token_address) {
        Ok(Some(_)) => true,
        _ => false,
    };
    if !associated_token_account_exists {
        let create_associated_token_account_instruction = spl_token::instruction::create_associated_token_account(
            &payer_account.pubkey(),
            &payer_account.pubkey(),
            &Pubkey::from_str("<TOKEN_MINT_ADDRESS>").unwrap(),
            &associated_token_address,
        );
        instructions.push(create_associated_token_account_instruction);
    }

    // Mint some tokens to the payer account
    let mint_amount = 1_000_000; // 1 million tokens
    let mint_to_instruction = mint_to(
        &token_program_id,
        &Pubkey::from_str("<TOKEN_MINT_ADDRESS>").unwrap(),
        &token_account_address,
        &payer_account.pubkey(),
        &[&payer_account.pubkey()],
        mint_amount,
    );
    instructions.push(mint_to_instruction);

    // Create the transaction
    let transaction = Instruction {
        program_id: system_program::id(),
        accounts: vec![],
        data: vec![],
    }
    .with_instructions(instructions);
    let recent_blockhash = rpc_client.get_recent_blockhash().unwrap().0;
    let mut transaction_signatures = rpc_client
        .send_transaction_with_config(
            &transaction,
            RpcSendTransactionConfig {
                preflight_commitment: None,
                skip_preflight: false,
                preflight_timeout: None,
