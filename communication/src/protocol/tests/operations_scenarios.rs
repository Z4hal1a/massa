// Copyright (c) 2021 MASSA LABS <info@massa.net>

// RUST_BACKTRACE=1 cargo test test_one_handshake -- --nocapture --test-threads=1

use super::tools;
use super::tools::protocol_test;
use crate::network::NetworkCommand;
use crate::protocol::ProtocolPoolEvent;
use models::{self, Address, Amount, OperationHashMap2, Slot};
use serial_test::serial;
use std::str::FromStr;
use std::time::Duration;

#[tokio::test]
#[serial]
async fn test_protocol_sends_valid_operations_it_receives_to_consensus() {
    let protocol_config = tools::create_protocol_config();
    protocol_test(
        protocol_config,
        async move |mut network_controller,
                    protocol_event_receiver,
                    protocol_command_sender,
                    protocol_manager,
                    mut protocol_pool_event_receiver| {
            // Create 1 node.
            let mut nodes = tools::create_and_connect_nodes(1, &mut network_controller).await;

            let creator_node = nodes.pop().expect("Failed to get node info.");

            // 1. Create an operation
            let operation = tools::create_operation();

            let expected_operation_id = operation.verify_integrity().unwrap();

            // 3. Send operation to protocol.
            network_controller
                .send_operations(creator_node.id, vec![operation])
                .await;

            // Check protocol sends operations to consensus.
            let received_operations = match tools::wait_protocol_pool_event(
                &mut protocol_pool_event_receiver,
                1000.into(),
                |evt| match evt {
                    evt @ ProtocolPoolEvent::ReceivedOperations { .. } => Some(evt),
                    _ => None,
                },
            )
            .await
            {
                Some(ProtocolPoolEvent::ReceivedOperations { operations, .. }) => operations,
                _ => panic!("Unexpected or no protocol pool event."),
            };
            assert!(received_operations.contains_key(&expected_operation_id));

            (
                network_controller,
                protocol_event_receiver,
                protocol_command_sender,
                protocol_manager,
                protocol_pool_event_receiver,
            )
        },
    )
    .await;
}

#[tokio::test]
#[serial]
async fn test_protocol_does_not_send_invalid_operations_it_receives_to_consensus() {
    let protocol_config = tools::create_protocol_config();
    protocol_test(
        protocol_config,
        async move |mut network_controller,
                    protocol_event_receiver,
                    protocol_command_sender,
                    protocol_manager,
                    mut protocol_pool_event_receiver| {
            // Create 1 node.
            let mut nodes = tools::create_and_connect_nodes(1, &mut network_controller).await;

            let creator_node = nodes.pop().expect("Failed to get node info.");

            // 1. Create an operation.
            let mut operation = tools::create_operation();

            // Change the fee, making the signature invalid.
            operation.content.fee = Amount::from_str("111").unwrap();

            // 3. Send operation to protocol.
            network_controller
                .send_operations(creator_node.id, vec![operation])
                .await;

            // Check protocol does not send operations to consensus.
            match tools::wait_protocol_pool_event(
                &mut protocol_pool_event_receiver,
                1000.into(),
                |evt| match evt {
                    evt @ ProtocolPoolEvent::ReceivedOperations { .. } => Some(evt),
                    _ => None,
                },
            )
            .await
            {
                Some(ProtocolPoolEvent::ReceivedOperations { .. }) => {
                    panic!("Protocol send invalid operations.")
                }
                _ => {}
            };

            (
                network_controller,
                protocol_event_receiver,
                protocol_command_sender,
                protocol_manager,
                protocol_pool_event_receiver,
            )
        },
    )
    .await;
}

#[tokio::test]
#[serial]
async fn test_protocol_propagates_operations_to_active_nodes() {
    let protocol_config = tools::create_protocol_config();
    protocol_test(
        protocol_config,
        async move |mut network_controller,
                    protocol_event_receiver,
                    mut protocol_command_sender,
                    protocol_manager,
                    mut protocol_pool_event_receiver| {
            // Create 2 nodes.
            let nodes = tools::create_and_connect_nodes(2, &mut network_controller).await;

            // 1. Create an operation
            let operation = tools::create_operation();

            // Send operation and wait for the protocol event,
            // just to be sure the nodes are connected before sending the propagate command.
            network_controller
                .send_operations(nodes[0].id, vec![operation.clone()])
                .await;
            let _received_operations = match tools::wait_protocol_pool_event(
                &mut protocol_pool_event_receiver,
                1000.into(),
                |evt| match evt {
                    evt @ ProtocolPoolEvent::ReceivedOperations { .. } => Some(evt),
                    _ => None,
                },
            )
            .await
            {
                Some(ProtocolPoolEvent::ReceivedOperations { operations, .. }) => operations,
                _ => panic!("Unexpected or no protocol pool event."),
            };

            let expected_operation_id = operation.verify_integrity().unwrap();

            let mut ops = OperationHashMap2::default();
            ops.insert(expected_operation_id.clone(), operation);
            protocol_command_sender
                .propagate_operations(ops)
                .await
                .unwrap();

            loop {
                match network_controller
                    .wait_command(1000.into(), |cmd| match cmd {
                        cmd @ NetworkCommand::SendOperations { .. } => Some(cmd),
                        _ => None,
                    })
                    .await
                {
                    Some(NetworkCommand::SendOperations { node, operations }) => {
                        let id = operations[0].verify_integrity().unwrap();
                        assert_eq!(id, expected_operation_id);
                        assert_eq!(nodes[1].id, node);
                        break;
                    }
                    _ => panic!("Unexpected or no network command."),
                };
            }
            (
                network_controller,
                protocol_event_receiver,
                protocol_command_sender,
                protocol_manager,
                protocol_pool_event_receiver,
            )
        },
    )
    .await;
}

#[tokio::test]
#[serial]
async fn test_protocol_propagates_operations_only_to_nodes_that_dont_know_avbout_it() {
    let protocol_config = tools::create_protocol_config();
    protocol_test(
        protocol_config,
        async move |mut network_controller,
                    protocol_event_receiver,
                    mut protocol_command_sender,
                    protocol_manager,
                    mut protocol_pool_event_receiver| {
            // Create 1 nodes.
            let nodes = tools::create_and_connect_nodes(1, &mut network_controller).await;

            // 1. Create an operation
            let operation = tools::create_operation();

            // Send operation and wait for the protocol event,
            // just to be sure the nodes are connected before sending the propagate command.
            network_controller
                .send_operations(nodes[0].id, vec![operation.clone()])
                .await;
            let _received_operations = match tools::wait_protocol_pool_event(
                &mut protocol_pool_event_receiver,
                1000.into(),
                |evt| match evt {
                    evt @ ProtocolPoolEvent::ReceivedOperations { .. } => Some(evt),
                    _ => None,
                },
            )
            .await
            {
                Some(ProtocolPoolEvent::ReceivedOperations { operations, .. }) => operations,
                _ => panic!("Unexpected or no protocol pool event."),
            };
            // create and connect a node that does not know about the endorsement
            let new_nodes = tools::create_and_connect_nodes(1, &mut network_controller).await;

            // wait for things to settle
            tokio::time::sleep(Duration::from_millis(250)).await;

            let expected_operation_id = operation.verify_integrity().unwrap();

            let mut ops = OperationHashMap2::default();
            ops.insert(expected_operation_id.clone(), operation);

            // send endorsement to protocol
            // it should be propagated only to the node that doesn't know about it
            protocol_command_sender
                .propagate_operations(ops)
                .await
                .unwrap();

            loop {
                match network_controller
                    .wait_command(1000.into(), |cmd| match cmd {
                        cmd @ NetworkCommand::SendOperations { .. } => Some(cmd),
                        _ => None,
                    })
                    .await
                {
                    Some(NetworkCommand::SendOperations { node, operations }) => {
                        let id = operations[0].verify_integrity().unwrap();
                        assert_eq!(id, expected_operation_id);
                        assert_eq!(new_nodes[0].id, node);
                        break;
                    }
                    _ => panic!("Unexpected or no network command."),
                };
            }
            (
                network_controller,
                protocol_event_receiver,
                protocol_command_sender,
                protocol_manager,
                protocol_pool_event_receiver,
            )
        },
    )
    .await;
}

#[tokio::test]
#[serial]
async fn test_protocol_does_not_propagates_operations_when_receiving_those_inside_a_block() {
    let protocol_config = tools::create_protocol_config();
    protocol_test(
        protocol_config,
        async move |mut network_controller,
                    protocol_event_receiver,
                    protocol_command_sender,
                    protocol_manager,
                    mut protocol_pool_event_receiver| {
            // Create 2 nodes.
            let mut nodes = tools::create_and_connect_nodes(2, &mut network_controller).await;

            let creator_node = nodes.pop().expect("Failed to get node info.");

            // 1. Create an operation
            let operation = tools::create_operation_with_keys(&creator_node.private_key);

            let address = Address::from_public_key(&creator_node.id.0).unwrap();
            let serialization_context = models::get_serialization_context();
            let thread = address.get_thread(serialization_context.parent_count);

            // 2. Create a block coming from node creator_node, and including the operation.
            let block = tools::create_block_with_operations(
                &creator_node.private_key,
                &creator_node.id.0,
                Slot::new(1, thread),
                vec![operation.clone()],
            );

            // 4. Send block to protocol.
            network_controller
                .send_block(creator_node.id, block.clone())
                .await;

            // 5. Check that the operation included in the block is not propagated.
            match tools::wait_protocol_pool_event(
                &mut protocol_pool_event_receiver,
                1000.into(),
                |evt| match evt {
                    evt @ ProtocolPoolEvent::ReceivedOperations { .. } => Some(evt),
                    _ => None,
                },
            )
            .await
            {
                None => panic!("Protocol did not send operations to pool."),
                Some(ProtocolPoolEvent::ReceivedOperations {
                    propagate,
                    operations,
                }) => {
                    let expected_id = operation.verify_integrity().unwrap();
                    assert_eq!(propagate, false);
                    assert_eq!(
                        operations
                            .get(&expected_id)
                            .unwrap()
                            .verify_integrity()
                            .unwrap(),
                        expected_id
                    );
                    assert_eq!(operations.len(), 1);
                }
                Some(_) => panic!("Unexpected protocol pool event."),
            }
            (
                network_controller,
                protocol_event_receiver,
                protocol_command_sender,
                protocol_manager,
                protocol_pool_event_receiver,
            )
        },
    )
    .await;
}
