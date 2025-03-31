use solana_program_test::{tokio, ProgramTest, ProgramTestBanksClientExt};
use solana_sdk::{pubkey::Pubkey, signer::Signer, transaction::Transaction};
use td_program_sdk::{instructions, seeds::PLAYER_SEED, PROGRAM_ID};

#[tokio::test]
async fn update_player_game_values() {
    let mut context = ProgramTest::new("tower_defense_program", PROGRAM_ID, None)
        .start_with_context()
        .await;

    let signer_pubkey = context.payer.pubkey();
    let seeds = [PLAYER_SEED, signer_pubkey.as_ref()];

    let (player, bump) = Pubkey::find_program_address(&seeds, &PROGRAM_ID);

    let wave_count = 10u8;
    let last_time_played = 1254142; // simulate last time played

    let init_player_ix = instructions::initialize_player(&player, &signer_pubkey, bump);
    let update_player_game_values_ix = instructions::update_player_game_values(
        &player,
        &signer_pubkey,
        last_time_played,
        wave_count,
    );

    let last_blockhash = context
        .banks_client
        .get_new_latest_blockhash(&context.last_blockhash)
        .await
        .unwrap();

    context.last_blockhash = last_blockhash;

    let tx = Transaction::new_signed_with_payer(
        &[init_player_ix, update_player_game_values_ix],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();
    let account = context.banks_client.get_account(player).await.unwrap();
    assert!(account.is_some());
}
