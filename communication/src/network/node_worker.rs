use super::{
    binders::{ReadBinder, WriteBinder},
    config::{NetworkConfig, CHANNEL_SIZE},
    messages::Message,
};
use crate::common::NodeId;
use crate::{error::CommunicationError, network::ConnectionClosureReason};
use crypto::hash::Hash;
use hang_monitor::{
    HangAnnotation, HangMonitorCommandSender, MonitoredComponentId, NetworkHangAnnotation,
    NodeHangAnnotation,
};
use models::{Block, BlockHeader};
use std::net::IpAddr;
use tokio::{sync::mpsc, time::timeout};

#[derive(Clone, Debug)]
pub enum NodeCommand {
    /// Send given peer list to node.
    SendPeerList(Vec<IpAddr>),
    /// Send that block to node.
    SendBlock(Block),
    /// Send the header of a block to a node.
    SendBlockHeader(BlockHeader),
    /// Ask for a block from that node.
    AskForBlock(Hash),
    /// Close the node worker.
    Close(ConnectionClosureReason),
    /// Block not founf
    BlockNotFound(Hash),
}

impl From<&NodeCommand> for NodeHangAnnotation {
    fn from(msg: &NodeCommand) -> Self {
        match msg {
            NodeCommand::SendPeerList(_) => NodeHangAnnotation::SendPeerList,
            NodeCommand::SendBlock(_) => NodeHangAnnotation::SendBlock,
            NodeCommand::SendBlockHeader(_) => NodeHangAnnotation::SendBlockHeader,
            NodeCommand::AskForBlock(_) => NodeHangAnnotation::CommandAskForBlock,
            NodeCommand::Close(_) => NodeHangAnnotation::Close,
            NodeCommand::BlockNotFound(_) => NodeHangAnnotation::CommandBlockNotFound,
        }
    }
}

/// Event types that node worker can emit
#[derive(Clone, Debug)]
pub enum NodeEventType {
    /// Node we are conneced to asked for advertized peers
    AskedPeerList,
    /// Node we are conneced to sent peer list
    ReceivedPeerList(Vec<IpAddr>),
    /// Node we are conneced to sent block
    ReceivedBlock(Block),
    /// Node we are conneced to sent block header
    ReceivedBlockHeader(BlockHeader),
    /// Node we are conneced to asks for a block.
    ReceivedAskForBlock(Hash),
    /// Connection with node was shut down for given reason
    Closed(ConnectionClosureReason),
    /// Didn't found given block,
    BlockNotFound(Hash),
}

/// Events node worker can emit.
/// Events are a tuple linking a node id to an event type
#[derive(Clone, Debug)]
pub struct NodeEvent(pub NodeId, pub NodeEventType);

impl From<&NodeEvent> for NetworkHangAnnotation {
    fn from(msg: &NodeEvent) -> Self {
        match msg.1 {
            NodeEventType::AskedPeerList => NetworkHangAnnotation::ReceivedAskedPeerList,
            NodeEventType::ReceivedPeerList(_) => NetworkHangAnnotation::ReceivedPeerList,
            NodeEventType::ReceivedBlock(_) => NetworkHangAnnotation::ReceivedBlock,
            NodeEventType::ReceivedBlockHeader(_) => NetworkHangAnnotation::ReceivedBlockHeader,
            NodeEventType::ReceivedAskForBlock(_) => NetworkHangAnnotation::ReceivedAskForBlock,
            NodeEventType::Closed(_) => NetworkHangAnnotation::ReceivedClosed,
            NodeEventType::BlockNotFound(_) => NetworkHangAnnotation::ReceivedBlockNotFound,
        }
    }
}

/// Manages connections
/// One worker per node.
pub struct NodeWorker {
    /// Protocol configuration.
    cfg: NetworkConfig,
    /// Node id associated to that worker.
    node_id: NodeId,
    /// Reader for incomming data.
    socket_reader: ReadBinder,
    /// Optional writer to send data.
    socket_writer_opt: Option<WriteBinder>,
    /// Channel to receive node commands.
    node_command_rx: mpsc::Receiver<NodeCommand>,
    /// Channel to send node events.
    node_event_tx: mpsc::Sender<NodeEvent>,
    /// Hang monitor.
    hang_monitor: Option<HangMonitorCommandSender>,
}

impl NodeWorker {
    /// Creates a new node worker
    ///
    /// # Arguments
    /// * cfg: Protocol configuration.
    /// * serialization_context: SerializationContext instance
    /// * node_id: Node id associated to that worker.
    /// * socket_reader: Reader for incomming data.
    /// * socket_writer: Writer for sending data.
    /// * node_command_rx: Channel to receive node commands.
    /// * node_event_tx: Channel to send node events.
    pub fn new(
        cfg: NetworkConfig,
        node_id: NodeId,
        socket_reader: ReadBinder,
        socket_writer: WriteBinder,
        node_command_rx: mpsc::Receiver<NodeCommand>,
        node_event_tx: mpsc::Sender<NodeEvent>,
        hang_monitor: Option<HangMonitorCommandSender>,
    ) -> NodeWorker {
        NodeWorker {
            cfg,
            node_id,
            socket_reader,
            socket_writer_opt: Some(socket_writer),
            node_command_rx,
            node_event_tx,
            hang_monitor,
        }
    }

    /// node event loop. Consumes self.
    pub async fn run_loop(mut self) -> Result<(), CommunicationError> {
        if let Some(monitor) = self.hang_monitor.as_mut() {
            // Register the worker with the hang-monitor.
            monitor
                .register(MonitoredComponentId::Node)
                .await
                .map_err(|_| {
                    CommunicationError::ChannelError("Register with hang-monitor failed".into())
                })?;
        }

        let (writer_command_tx, mut writer_command_rx) = mpsc::channel::<Message>(CHANNEL_SIZE);
        let (writer_event_tx, mut writer_event_rx) = mpsc::channel::<bool>(1);
        let mut socket_writer =
            self.socket_writer_opt
                .take()
                .ok_or(CommunicationError::GeneralProtocolError(
                    "NodeWorker call run_loop more than once".to_string(),
                ))?;
        let write_timeout = self.cfg.message_timeout;
        let node_writer_handle = tokio::spawn(async move {
            let mut clean_exit = true;
            loop {
                match writer_command_rx.recv().await {
                    Some(msg) => {
                        if let Err(_) =
                            timeout(write_timeout.to_duration(), socket_writer.send(&msg)).await
                        {
                            clean_exit = false;
                            break;
                        }
                    }
                    None => break,
                }
            }
            writer_event_tx
                .send(clean_exit)
                .await
                .expect("writer_evt_tx died"); //in a spawned task
        });

        let mut ask_peer_list_interval =
            tokio::time::interval(self.cfg.ask_peer_list_interval.to_duration());
        let mut exit_reason = ConnectionClosureReason::Normal;
        loop {
            if let Some(monitor) = self.hang_monitor.as_mut() {
                // Notify the hang-monitor we are about to wait on a select.
                monitor.notify_wait().await.map_err(|_| {
                    CommunicationError::ChannelError(
                        "Wait notification to hang-monitor failed".into(),
                    )
                })?;
            }
            tokio::select! {
                // incoming socket data
                res = self.socket_reader.next() => match res {
                    Ok(Some((_, msg))) => {
                        if let Some(monitor) = self.hang_monitor.as_mut() {
                            monitor
                                .notify_activity(HangAnnotation::Node((&msg).into()))
                                .await
                                .map_err(|_| {
                                    CommunicationError::ChannelError("Activity notification to hang-monitor failed".into())
                                })?;
                        }
                        match msg {
                            Message::Block(block) => self.node_event_tx.send(
                                    NodeEvent(self.node_id, NodeEventType::ReceivedBlock(block))
                                ).await.map_err(|_| CommunicationError::ChannelError("failed to send received block event".into()))?,
                            Message::BlockHeader(header) => self.node_event_tx.send(
                                    NodeEvent(self.node_id, NodeEventType::ReceivedBlockHeader(header))
                                ).await.map_err(|_| CommunicationError::ChannelError("failed to send received block header event".into()))?,
                            Message::AskForBlock(hash) => self.node_event_tx.send(
                                    NodeEvent(self.node_id, NodeEventType::ReceivedAskForBlock(hash))
                                ).await.map_err(|_| CommunicationError::ChannelError("failed to send received block header event".into()))?,
                            Message::PeerList(pl) =>  self.node_event_tx.send(
                                    NodeEvent(self.node_id, NodeEventType::ReceivedPeerList(pl))
                                ).await.map_err(|_| CommunicationError::ChannelError("failed to send received peers list event".into()))?,
                            Message::AskPeerList => self.node_event_tx.send(
                                    NodeEvent(self.node_id, NodeEventType::AskedPeerList)
                                ).await.map_err(|_| CommunicationError::ChannelError("failed to send asked block event".into()))?,
                            Message::BlockNotFound(hash) => self.node_event_tx.send(
                                NodeEvent(self.node_id, NodeEventType::BlockNotFound(hash))
                            ).await.map_err(|_| CommunicationError::ChannelError("failed to send block not found event".into()))?,
                            _ => {  // wrong message
                                exit_reason = ConnectionClosureReason::Failed;
                                break;
                            },
                        }
                    },
                    Ok(None)=> break, // peer closed cleanly
                    Err(_) => {  //stream error
                        exit_reason = ConnectionClosureReason::Failed;
                        break;
                    },
                },

                // node command
                cmd = self.node_command_rx.recv() => {
                    if let Some(monitor) = self.hang_monitor.as_mut() {
                        if let Some(cmd) = cmd.as_ref() {
                            monitor
                                .notify_activity(HangAnnotation::Node(cmd.into()))
                                .await
                                .map_err(|_| {
                                    CommunicationError::ChannelError("Activity notification to hang-monitor failed".into())
                                })?;
                        }
                    }

                    match cmd {
                        Some(NodeCommand::Close(r)) => {
                            exit_reason = r;
                            break;
                        },
                        Some(NodeCommand::SendPeerList(ip_vec)) => {
                            writer_command_tx.send(Message::PeerList(ip_vec)).await.map_err(
                                |_| CommunicationError::ChannelError("send peer list node command send failed".into())
                            )?;
                        },
                        Some(NodeCommand::SendBlockHeader(header)) => {
                            writer_command_tx.send(Message::BlockHeader(header)).await.map_err(
                                |_| CommunicationError::ChannelError("send peer block header node command send failed".into())
                            )?;
                        },
                        Some(NodeCommand::SendBlock(block)) => {
                            writer_command_tx.send(Message::Block(block)).await.map_err(
                                |_| CommunicationError::ChannelError("send peer block node command send failed".into())
                            )?;
                        },
                        Some(NodeCommand::AskForBlock(block)) => {
                            writer_command_tx.send(Message::AskForBlock(block)).await.map_err(
                                |_| CommunicationError::ChannelError("ask peer block node command send failed".into())
                            )?;
                        },
                        Some(NodeCommand::BlockNotFound(hash)) =>  {
                            writer_command_tx.send(Message::BlockNotFound(hash)).await.map_err(
                                |_| CommunicationError::ChannelError("send peer block not found node command send failed".into())
                            )?;
                        },
                        None => {
                            return Err(CommunicationError::UnexpectedProtocolControllerClosureError);
                        },
                    }
                },

                // writer event
                evt = writer_event_rx.recv() => match evt {
                    Some(s) => {
                        if !s {
                            exit_reason = ConnectionClosureReason::Failed;
                        }
                        break;
                    },
                    None => break
                },

                _ = ask_peer_list_interval.tick() => {
                    if let Some(monitor) = self.hang_monitor.as_mut() {
                        monitor
                            .notify_activity(HangAnnotation::Node(NodeHangAnnotation::SendAskPeerList))
                            .await
                            .map_err(|_| {
                                CommunicationError::ChannelError("Activity notification to hang-monitor failed".into())
                            })?;
                    }

                    debug!("timer-based asking node_id={:?} for peer list", self.node_id);
                    massa_trace!("timer_ask_peer_list", {"node_id": self.node_id});
                    writer_command_tx.send(Message::AskPeerList).await.map_err(
                        |_| CommunicationError::ChannelError("writer send ask peer list failed".into())
                    )?;
                }
            }
        }

        // close writer
        drop(writer_command_tx);
        while let Some(_) = writer_event_rx.recv().await {}
        node_writer_handle.await?;

        if let Some(monitor) = self.hang_monitor.as_mut() {
            monitor
                .notify_activity(HangAnnotation::Node(NodeHangAnnotation::Closed))
                .await
                .map_err(|_| {
                    CommunicationError::ChannelError(
                        "Activity notification to hang-monitor failed".into(),
                    )
                })?;
        }

        // notify protocol controller of closure
        self.node_event_tx
            .send(NodeEvent(self.node_id, NodeEventType::Closed(exit_reason)))
            .await
            .map_err(|_| {
                CommunicationError::ChannelError("node closing event send failed".into())
            })?;
        Ok(())
    }
}
