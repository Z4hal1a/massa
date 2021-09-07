// Copyright (c) 2021 MASSA LABS <info@massa.net>

use std::collections::HashMap;

use super::tools;
use crate::tests::{
    block_factory::BlockFactory,
    tools::{create_endorsement, generate_ledger_file},
};
use crypto::hash::Hash;
use models::{BlockId, Slot};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_old_stale_not_propagated_and_discarded() {
    let ledger_file = generate_ledger_file(&HashMap::new());
    let staking_keys: Vec<crypto::signature::PrivateKey> = (0..1)
        .map(|_| crypto::generate_random_private_key())
        .collect();
    let roll_counts_file = tools::generate_default_roll_counts_file(staking_keys.clone());
    let staking_file = tools::generate_staking_keys_file(&staking_keys);
    let mut cfg = tools::default_consensus_config(
        ledger_file.path(),
        roll_counts_file.path(),
        staking_file.path(),
    );
    cfg.t0 = 1000.into();
    cfg.future_block_processing_max_periods = 50;
    cfg.max_future_processing_blocks = 10;

    tools::consensus_without_pool_test(
        cfg.clone(),
        None,
        async move |protocol_controller, consensus_command_sender, consensus_event_receiver| {
            let parents: Vec<BlockId> = consensus_command_sender
                .get_block_graph_status()
                .await
                .expect("could not get block graph status")
                .best_parents
                .iter()
                .map(|(b, _p)| *b)
                .collect();

            let mut block_factory =
                BlockFactory::start_block_factory(parents.clone(), protocol_controller);
            block_factory.set_creator(staking_keys[0]);
            block_factory.set_slot(Slot::new(1, 0));

            let (hash_1, _) = block_factory.create_and_receive_block(true).await;

            block_factory.set_slot(Slot::new(1, 1));
            block_factory.create_and_receive_block(true).await;

            block_factory.set_slot(Slot::new(1, 0));
            block_factory.set_parents(vec![hash_1, parents[0]]);
            let (hash_3, _) = block_factory.create_and_receive_block(false).await;

            // Old stale block was discarded.
            let status = consensus_command_sender
                .get_block_graph_status()
                .await
                .expect("could not get block graph status");
            assert_eq!(status.discarded_blocks.map.len(), 1);
            assert!(status.discarded_blocks.map.get(&hash_3).is_some());
            (
                block_factory.give_protocol_controller(),
                consensus_command_sender,
                consensus_event_receiver,
            )
        },
    )
    .await;
}

#[tokio::test]
#[serial]
async fn test_block_not_processed_multiple_times() {
    let ledger_file = generate_ledger_file(&HashMap::new());
    let staking_keys: Vec<crypto::signature::PrivateKey> = (0..1)
        .map(|_| crypto::generate_random_private_key())
        .collect();
    let roll_counts_file = tools::generate_default_roll_counts_file(staking_keys.clone());
    let staking_file = tools::generate_staking_keys_file(&staking_keys);
    let mut cfg = tools::default_consensus_config(
        ledger_file.path(),
        roll_counts_file.path(),
        staking_file.path(),
    );
    cfg.t0 = 500.into();
    cfg.future_block_processing_max_periods = 50;
    cfg.max_future_processing_blocks = 10;

    tools::consensus_without_pool_test(
        cfg.clone(),
        None,
        async move |protocol_controller, consensus_command_sender, consensus_event_receiver| {
            let parents: Vec<BlockId> = consensus_command_sender
                .get_block_graph_status()
                .await
                .expect("could not get block graph status")
                .best_parents
                .iter()
                .map(|(b, _p)| *b)
                .collect();

            let mut block_factory =
                BlockFactory::start_block_factory(parents.clone(), protocol_controller);
            block_factory.set_creator(staking_keys[0]);
            block_factory.set_slot(Slot::new(1, 0));
            let (_, block_1) = block_factory.create_and_receive_block(true).await;

            // Send it again, it should not be propagated.
            block_factory.receieve_block(false, block_1.clone()).await;

            // Send it again, it should not be propagated.
            block_factory.receieve_block(false, block_1.clone()).await;

            // Block was not discarded.
            let status = consensus_command_sender
                .get_block_graph_status()
                .await
                .expect("could not get block graph status");
            assert_eq!(status.discarded_blocks.map.len(), 0);
            (
                block_factory.give_protocol_controller(),
                consensus_command_sender,
                consensus_event_receiver,
            )
        },
    )
    .await;
}

#[tokio::test]
#[serial]
async fn test_queuing() {
    let ledger_file = generate_ledger_file(&HashMap::new());
    let staking_keys: Vec<crypto::signature::PrivateKey> = (0..1)
        .map(|_| crypto::generate_random_private_key())
        .collect();
    let roll_counts_file = tools::generate_default_roll_counts_file(staking_keys.clone());
    let staking_file = tools::generate_staking_keys_file(&staking_keys);
    let mut cfg = tools::default_consensus_config(
        ledger_file.path(),
        roll_counts_file.path(),
        staking_file.path(),
    );
    cfg.t0 = 1000.into();
    cfg.future_block_processing_max_periods = 50;
    cfg.max_future_processing_blocks = 10;

    tools::consensus_without_pool_test(
        cfg.clone(),
        None,
        async move |protocol_controller, consensus_command_sender, consensus_event_receiver| {
            let parents: Vec<BlockId> = consensus_command_sender
                .get_block_graph_status()
                .await
                .expect("could not get block graph status")
                .best_parents
                .iter()
                .map(|(b, _p)| *b)
                .collect();

            let mut block_factory =
                BlockFactory::start_block_factory(parents.clone(), protocol_controller);
            block_factory.set_creator(staking_keys[0]);
            block_factory.set_slot(Slot::new(3, 0));

            let (hash_1, _) = block_factory.create_and_receive_block(false).await;

            block_factory.set_slot(Slot::new(4, 0));
            block_factory.set_parents(vec![hash_1.clone(), parents[1]]);

            block_factory.create_and_receive_block(false).await;

            // Blocks were queued, not discarded.
            let status = consensus_command_sender
                .get_block_graph_status()
                .await
                .expect("could not get block graph status");
            assert_eq!(status.discarded_blocks.map.len(), 0);
            (
                block_factory.give_protocol_controller(),
                consensus_command_sender,
                consensus_event_receiver,
            )
        },
    )
    .await;
}

#[tokio::test]
#[serial]
async fn test_double_staking_does_not_propagate() {
    let ledger_file = generate_ledger_file(&HashMap::new());
    let staking_keys: Vec<crypto::signature::PrivateKey> = (0..1)
        .map(|_| crypto::generate_random_private_key())
        .collect();
    let roll_counts_file = tools::generate_default_roll_counts_file(staking_keys.clone());
    let staking_file = tools::generate_staking_keys_file(&staking_keys);
    let mut cfg = tools::default_consensus_config(
        ledger_file.path(),
        roll_counts_file.path(),
        staking_file.path(),
    );
    cfg.t0 = 1000.into();
    cfg.future_block_processing_max_periods = 50;
    cfg.max_future_processing_blocks = 10;

    tools::consensus_without_pool_test(
        cfg.clone(),
        None,
        async move |protocol_controller, consensus_command_sender, consensus_event_receiver| {
            let parents: Vec<BlockId> = consensus_command_sender
                .get_block_graph_status()
                .await
                .expect("could not get block graph status")
                .best_parents
                .iter()
                .map(|(b, _p)| *b)
                .collect();

            let mut block_factory =
                BlockFactory::start_block_factory(parents.clone(), protocol_controller);
            block_factory.set_creator(staking_keys[0]);
            block_factory.set_slot(Slot::new(1, 0));
            let (_, mut block_1) = block_factory.create_and_receive_block(true).await;

            // Same creator, same slot, different block
            block_1.header.content.operation_merkle_root = Hash::hash(&"hello world".as_bytes());
            let block = block_factory.sign_header(block_1.header.content);

            // Note: currently does propagate, see #190.
            block_factory.receieve_block(true, block).await;

            // Block was not discarded.
            let status = consensus_command_sender
                .get_block_graph_status()
                .await
                .expect("could not get block graph status");
            assert_eq!(status.discarded_blocks.map.len(), 0);
            (
                block_factory.give_protocol_controller(),
                consensus_command_sender,
                consensus_event_receiver,
            )
        },
    )
    .await;
}
