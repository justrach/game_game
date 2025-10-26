use solana_program_test::*;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

#[tokio::test]
async fn test_player_initialization() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "onchain_rpg",
        program_id,
        processor!(onchain_rpg::process_instruction),
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Derive player PDA
    let (player_pda, _bump) = Pubkey::find_program_address(
        &[b"player", payer.pubkey().as_ref()],
        &program_id,
    );

    // Create initialize player instruction
    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(player_pda, false),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        ],
        data: vec![0], // Discriminator 0 = player_initialize
    };

    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);

    let result = banks_client.process_transaction(transaction).await;
    assert!(result.is_ok(), "Player initialization failed");

    // Verify player account was created
    let player_account = banks_client.get_account(player_pda).await.unwrap();
    assert!(player_account.is_some(), "Player account not created");
}

#[tokio::test]
async fn test_treasury_initialization() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "onchain_rpg",
        program_id,
        processor!(onchain_rpg::process_instruction),
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Derive treasury PDA
    let (treasury_pda, _bump) = Pubkey::find_program_address(&[b"treasury"], &program_id);

    // Create initialize treasury instruction
    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(treasury_pda, false),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        ],
        data: vec![12], // Discriminator 12 = initialize_treasury
    };

    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);

    let result = banks_client.process_transaction(transaction).await;
    assert!(result.is_ok(), "Treasury initialization failed");
}

#[tokio::test]
async fn test_full_game_flow() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "onchain_rpg",
        program_id,
        processor!(onchain_rpg::process_instruction),
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // 1. Initialize treasury
    let (treasury_pda, _) = Pubkey::find_program_address(&[b"treasury"], &program_id);
    let init_treasury_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(treasury_pda, false),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        ],
        data: vec![12],
    };

    let mut tx = Transaction::new_with_payer(&[init_treasury_ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(tx).await.unwrap();

    // 2. Initialize player
    let (player_pda, _) = Pubkey::find_program_address(
        &[b"player", payer.pubkey().as_ref()],
        &program_id,
    );
    let init_player_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(player_pda, false),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        ],
        data: vec![0],
    };

    let mut tx = Transaction::new_with_payer(&[init_player_ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(tx).await.unwrap();

    // 3. Buy hero
    let hero_index = 0u64;
    let (hero_pda, _) = Pubkey::find_program_address(
        &[b"hero", payer.pubkey().as_ref(), &hero_index.to_le_bytes()],
        &program_id,
    );
    let buy_hero_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(player_pda, false),
            AccountMeta::new(hero_pda, false),
            AccountMeta::new(treasury_pda, false),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        ],
        data: vec![1],
    };

    let mut tx = Transaction::new_with_payer(&[buy_hero_ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], recent_blockhash);
    let result = banks_client.process_transaction(tx).await;
    assert!(result.is_ok(), "Buy hero failed");

    println!("✅ Full game flow test passed!");
}
