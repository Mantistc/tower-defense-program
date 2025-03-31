use solana_program_test::{tokio, ProgramTest, ProgramTestBanksClientExt};
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction};
use td_program_sdk::{instructions, seeds::PLAYER_SEED, PROGRAM_ID};

#[tokio::test]
async fn update_player_authority() {
    let mut context = ProgramTest::new("tower_defense_program", PROGRAM_ID, None)
        .start_with_context()
        .await;

    let signer_pubkey = context.payer.pubkey();
    let seeds = [PLAYER_SEED, signer_pubkey.as_ref()];
    let new_auth = Keypair::new();
    let (player, bump) = Pubkey::find_program_address(&seeds, &PROGRAM_ID);

    let init_player_ix = instructions::initialize_player(&player, &signer_pubkey, bump);
    let update_player_auth =
        instructions::update_player_authority(&player, &signer_pubkey, &new_auth.pubkey());

    let last_blockhash = context
        .banks_client
        .get_new_latest_blockhash(&context.last_blockhash)
        .await
        .unwrap();

    context.last_blockhash = last_blockhash;

    let tx = Transaction::new_signed_with_payer(
        &[init_player_ix, update_player_auth],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await.unwrap();

    let account = context.banks_client.get_account(player).await.unwrap();

    assert!(account.is_some());
}
