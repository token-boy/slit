use slit_api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "slit_program",
        slit_api::ID,
        processor!(slit_program::process_instruction),
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn run_test() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;

    let board_id = uuid::Uuid::new_v4().into_bytes();
    let board = board_pda(&board_id).0;
    let player = player_pda(payer.pubkey()).0;

    // Submit register transaction.
    let ix = register(payer.pubkey(), player);
    let tx: Transaction =
        Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // Submit initialize transaction.
    let ix = initialize(payer.pubkey());
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // Verify board was initialized.
    let board_account = banks.get_account(board).await.unwrap().unwrap();
    assert_eq!(board_account.owner, slit_api::ID);
}
