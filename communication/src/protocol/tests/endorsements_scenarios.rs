// Copyright (c) 2021 MASSA LABS <info@massa.net>

//RUST_BACKTRACE=1 cargo test test_one_handshake -- --nocapture --test-threads=1

use super::tools;
use super::tools::protocol_test;
use crate::network::NetworkCommand;
use crate::protocol::ProtocolPoolEvent;
use models::Slot;
use serial_test::serial;
use std::collections::HashMap;
use std::time::Duration;

#[tokio::test]
#[serial]
async fn test_protocol_sends_valid_endorsements_it_receives_to_pool() {
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

            // 1. Create an endorsement
            let endorsement = tools::create_endorsement();

            let expected_endorsement_id = endorsement.compute_endorsement_id().unwrap();

            // 3. Send endorsement to protocol.
            network_controller
                .send_endorsements(creator_node.id, vec![endorsement])
                .await;

            // Check protocol sends endorsements to pool.
            let received_endorsements = match tools::wait_protocol_pool_event(
                &mut protocol_pool_event_receiver,
                1000.into(),
                |evt| match evt {
                    evt @ ProtocolPoolEvent::ReceivedEndorsements { .. } => Some(evt),
                    _ => None,
                },
            )
            .await
            {
                Some(ProtocolPoolEvent::ReceivedEndorsements { endorsements, .. }) => endorsements,
                _ => panic!("Unexpected or no protocol pool event."),
            };
            assert!(received_endorsements.contains_key(&expected_endorsement_id));

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
async fn test_protocol_does_not_send_invalid_endorsements_it_receives_to_pool() {
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

            // 1. Create an endorsement.
            let mut endorsement = tools::create_endorsement();

            // Change the slot, making the signature invalid.
            endorsement.content.slot = Slot::new(1, 1);

            // 3. Send operation to protocol.
            network_controller
                .send_endorsements(creator_node.id, vec![endorsement])
                .await;

            // Check protocol does not send endorsements to pool.
            match tools::wait_protocol_pool_event(
                &mut protocol_pool_event_receiver,
                1000.into(),
                |evt| match evt {
                    evt @ ProtocolPoolEvent::ReceivedEndorsements { .. } => Some(evt),
                    _ => None,
                },
            )
            .await
            {
                Some(ProtocolPoolEvent::ReceivedEndorsements { .. }) => {
                    panic!("Protocol send invalid endorsements.")
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
async fn test_protocol_propagates_endorsements_to_active_nodes() {
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

            // 1. Create an endorsement
            let endorsement = tools::create_endorsement();

            // Send endorsement and wait for the protocol event,
            // just to be sure the nodes are connected before sending the propagate command.
            network_controller
                .send_endorsements(nodes[0].id, vec![endorsement.clone()])
                .await;
            let _received_endorsements = match tools::wait_protocol_pool_event(
                &mut protocol_pool_event_receiver,
                1000.into(),
                |evt| match evt {
                    evt @ ProtocolPoolEvent::ReceivedEndorsements { .. } => Some(evt),
                    _ => None,
                },
            )
            .await
            {
                Some(ProtocolPoolEvent::ReceivedEndorsements { endorsements, .. }) => endorsements,
                _ => panic!("Unexpected or no protocol pool event."),
            };

            let expected_endorsement_id = endorsement.compute_endorsement_id().unwrap();

            let mut ops = HashMap::new();
            ops.insert(expected_endorsement_id.clone(), endorsement);
            protocol_command_sender
                .propagate_endorsements(ops)
                .await
                .unwrap();

            loop {
                match network_controller
                    .wait_command(1000.into(), |cmd| match cmd {
                        cmd @ NetworkCommand::SendEndorsements { .. } => Some(cmd),
                        _ => None,
                    })
                    .await
                {
                    Some(NetworkCommand::SendEndorsements { node, endorsements }) => {
                        let id = endorsements[0].compute_endorsement_id().unwrap();
                        assert_eq!(id, expected_endorsement_id);
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
async fn test_protocol_propagates_endorsements_only_to_nodes_that_dont_know_about_it() {
    let protocol_config = tools::create_protocol_config();
    protocol_test(
        protocol_config,
        async move |mut network_controller,
                    protocol_event_receiver,
                    mut protocol_command_sender,
                    protocol_manager,
                    mut protocol_pool_event_receiver| {
            // Create 1 node.
            let nodes = tools::create_and_connect_nodes(1, &mut network_controller).await;

            // 1. Create an endorsement
            let endorsement = tools::create_endorsement();

            // Send endorsement and wait for the protocol event,
            // just to be sure the nodes are connected before sending the propagate command.
            network_controller
                .send_endorsements(nodes[0].id, vec![endorsement.clone()])
                .await;
            let _received_endorsements = match tools::wait_protocol_pool_event(
                &mut protocol_pool_event_receiver,
                1000.into(),
                |evt| match evt {
                    evt @ ProtocolPoolEvent::ReceivedEndorsements { .. } => Some(evt),
                    _ => None,
                },
            )
            .await
            {
                Some(ProtocolPoolEvent::ReceivedEndorsements { endorsements, .. }) => endorsements,
                _ => panic!("Unexpected or no protocol pool event."),
            };

            // create and connect a node that does not know about the endorsement
            let new_nodes = tools::create_and_connect_nodes(1, &mut network_controller).await;

            // wait for things to settle
            tokio::time::sleep(Duration::from_millis(250)).await;

            let expected_endorsement_id = endorsement.compute_endorsement_id().unwrap();

            // send the endorsement to protocol
            // it should propagate it to nodes that don't know about it
            let mut ops = HashMap::new();
            ops.insert(expected_endorsement_id.clone(), endorsement);
            protocol_command_sender
                .propagate_endorsements(ops)
                .await
                .unwrap();

            loop {
                match network_controller
                    .wait_command(1000.into(), |cmd| match cmd {
                        cmd @ NetworkCommand::SendEndorsements { .. } => Some(cmd),
                        _ => None,
                    })
                    .await
                {
                    Some(NetworkCommand::SendEndorsements { node, endorsements }) => {
                        let id = endorsements[0].compute_endorsement_id().unwrap();
                        assert_eq!(id, expected_endorsement_id);
                        assert_eq!(new_nodes[0].id, node);
                        break;
                    }
                    None => panic!("no network command"),
                    Some(cmd) => panic!("Unexpected network command.{:?}", cmd),
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
async fn test_protocol_does_not_propagates_endorsements_when_receiving_those_inside_a_header() {
    let protocol_config = tools::create_protocol_config();
    protocol_test(
        protocol_config,
        async move |mut network_controller,
                    protocol_event_receiver,
                    protocol_command_sender,
                    protocol_manager,
                    protocol_pool_event_receiver| {
            // Create 2 nodes.
            let mut nodes = tools::create_and_connect_nodes(2, &mut network_controller).await;

            // 1. Create an endorsement
            let endorsement = tools::create_endorsement();

            let creator_node = nodes.pop().expect("Failed to get node info.");

            // 2. Create a block coming from node creator_node.
            let mut block = tools::create_block(&creator_node.private_key, &creator_node.id.0);

            // 3. Add endorsement to block
            block.header.content.endorsements = vec![endorsement.clone()];

            // 4. Send header to protocol.
            network_controller
                .send_header(creator_node.id, block.header.clone())
                .await;

            let expected_endorsement_id = endorsement.compute_endorsement_id().unwrap();

            // 5. Check that the endorsements included in the header are not propagated.
            loop {
                match network_controller
                    .wait_command(1000.into(), |cmd| match cmd {
                        cmd @ NetworkCommand::SendEndorsements { .. } => Some(cmd),
                        _ => None,
                    })
                    .await
                {
                    Some(NetworkCommand::SendEndorsements {
                        node: _node,
                        endorsements,
                    }) => {
                        let id = endorsements[0].compute_endorsement_id().unwrap();
                        assert_eq!(id, expected_endorsement_id);
                        panic!("Unexpected propagation of endorsement received inside header.")
                    }
                    Some(_) => panic!("Unpexted network command."),
                    None => break,
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
