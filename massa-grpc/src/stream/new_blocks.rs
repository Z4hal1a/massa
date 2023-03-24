use crate::error::GrpcError;
use crate::service::MassaGrpcService;
use futures_util::StreamExt;
use massa_models::block::SecureShareBlock;
use massa_proto::massa::api::v1::{self as grpc};
use massa_proto::massa::api::v1::{NewBlocksStreamRequest, NewBlocksStreamResponse};
use std::pin::Pin;
use tokio::select;
use tonic::codegen::futures_core;
use tonic::{Request, Streaming};
use tracing::log::error;

/// type declaration for NewBlocksStream
pub type NewBlocksStream = Pin<
    Box<
        dyn futures_core::Stream<Item = Result<NewBlocksStreamResponse, tonic::Status>>
            + Send
            + 'static,
    >,
>;

pub(crate) async fn new_blocks(
    grpc: &MassaGrpcService,
    request: Request<Streaming<NewBlocksStreamRequest>>,
) -> Result<NewBlocksStream, GrpcError> {
    let (tx, rx) = tokio::sync::mpsc::channel(grpc.grpc_config.max_channel_size);
    let mut in_stream = request.into_inner();
    let mut subscriber = grpc.consensus_channels.block_sender.subscribe();

    tokio::spawn(async move {
        // let mut request_id = request.id;

        // todo request id

        loop {
            select! {
                 event = subscriber.recv() => {

                    match event {
                        Ok(share_block) => {
                            let massa_block = share_block as SecureShareBlock;

                            let header = massa_block.content.header.into();
                            let operations = massa_block.content.operations.into_iter().map(|operation| operation.to_string()).collect();

                            if let Err(e) = tx.send(Ok(NewBlocksStreamResponse {
                                    id: "todo".to_string(),
                                    block: Some(grpc::Block {
                                        header: Some(header),
                                        operations
                                    })
                            })).await {
                                error!("failed to send operation : {}", e);
                                break;
                            }
                        },
                        Err(e) => {error!("error on receive block : {}", e)}
                    }
                },
                res = in_stream.next() => {
                    match res {
                        Some(res) => {
                            match res {
                                Ok(_data) => {
                                    // nothing to do
                                },
                                Err(e) => {
                                    error!("{}", e);
                                    break;
                                }
                            }
                        },
                        None => {
                            // client disconnected
                            break;
                        },
                    }
                }
            }
        }
    });

    let out_stream = tokio_stream::wrappers::ReceiverStream::new(rx);
    Ok(Box::pin(out_stream) as NewBlocksStream)
}
