use solana_program_test::{tokio, ProgramTest, ProgramTestBanksClientExt};
use solana_sdk::{pubkey::Pubkey, signer::Signer, transaction::Transaction};
use td_program_sdk::{instructions, seeds::PLAYER_SEED, states::Player, PROGRAM_ID};

#[tokio::test]
async fn initialize_player_test() {
    let mut context = ProgramTest::new("tower_defense_program", PROGRAM_ID, None)
        .start_with_context()
        .await;

    let payer_pubkey = context.payer.pubkey();
    let seeds = [PLAYER_SEED, payer_pubkey.as_ref()];

    let (player, bump) = Pubkey::find_program_address(&seeds, &PROGRAM_ID);

    let init_player_ix = instructions::initialize_player(&player, &payer_pubkey, bump);

    let last_blockhash = context
        .banks_client
        .get_new_latest_blockhash(&context.last_blockhash)
        .await
        .unwrap();

    context.last_blockhash = last_blockhash;

    let tx = Transaction::new_signed_with_payer(
        &[init_player_ix],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    let account = context.banks_client.get_account(player).await.unwrap();

    assert!(account.is_some());

    let player_account = Player::unpack(&account.clone().unwrap().data).unwrap();
    let player_authority = Pubkey::new_from_array(player_account.authority);

    assert!(player_authority == payer_pubkey);
}
