use solana_program_test::*;
use solana_sdk::{transaction::Transaction, pubkey::Pubkey, signer::Signer};
use solana_sdk::signature::Keypair;
use solana_sdk::account::Account;
use solana_sdk::system_program;
use solana_program::borsh::BorshSerialize;
use solana_program::instruction::Instruction;
use solana_program::program_pack::Pack;
use borsh::BorshDeserialize;
use crate::instruction::ReviewInstruction;
use crate::state::AccountState;

#[tokio::test]
async fn test_add_and_update_review() {
    // Create a program test environment
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "review_program",
        program_id,
        processor!(process_instruction),
    );

    // Create a keypair for the user
    let user = Keypair::new();
    let pda_account = Keypair::new();
    let rent = Rent::get().unwrap();
    let account_len = 1000;
    let rent_lamports = rent.minimum_balance(account_len);

    program_test.add_account(
        pda_account.pubkey(),
        Account::new(rent_lamports, account_len, &program_id),
    );

    let (pda, bump_seed) = Pubkey::find_program_address(
        &[user.pubkey().as_ref(), b"title".as_ref()],
        &program_id,
    );

    // Start the program test
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Add review instruction
    let add_review_instruction = ReviewInstruction::AddReview {
        title: "Awesome Restaurant".to_string(),
        rating: 5,
        description: "Great food and service".to_string(),
        location: "Downtown".to_string(),
    };
    let mut instruction_data = Vec::new();
    add_review_instruction.serialize(&mut instruction_data).unwrap();

    let add_review_ix = Instruction::new_with_bytes(
        program_id,
        &instruction_data,
        vec![
            AccountMeta::new(user.pubkey(), true),
            AccountMeta::new(pda_account.pubkey(), false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
    );

    let mut transaction = Transaction::new_with_payer(
        &[add_review_ix],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &user], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify review was added
    let pda_account_data = banks_client
        .get_account(pda_account.pubkey())
        .await
        .unwrap()
        .unwrap();
    let account_state = AccountState::try_from_slice(&pda_account_data.data).unwrap();

    assert_eq!(account_state.title, "Awesome Restaurant");
    assert_eq!(account_state.rating, 5);
    assert_eq!(account_state.description, "Great food and service");
    assert_eq!(account_state.location, "Downtown");

    // Update review instruction
    let update_review_instruction = ReviewInstruction::UpdateReview {
        title: "Awesome Restaurant".to_string(),
        rating: 4,
        description: "Food was great but service was slow".to_string(),
        location: "Downtown".to_string(),
    };
    instruction_data.clear();
    update_review_instruction.serialize(&mut instruction_data).unwrap();

    let update_review_ix = Instruction::new_with_bytes(
        program_id,
        &instruction_data,
        vec![
            AccountMeta::new(user.pubkey(), true),
            AccountMeta::new(pda_account.pubkey(), false),
        ],
    );

    let mut transaction = Transaction::new_with_payer(
        &[update_review_ix],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &user], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify review was updated
    let pda_account_data = banks_client
        .get_account(pda_account.pubkey())
        .await
        .unwrap()
        .unwrap();
    let updated_account_state = AccountState::try_from_slice(&pda_account_data.data).unwrap();

    assert_eq!(updated_account_state.title, "Awesome Restaurant");
    assert_eq!(updated_account_state.rating, 4);
    assert_eq!(updated_account_state.description, "Food was great but service was slow");
    assert_eq!(updated_account_state.location, "Downtown");
}
