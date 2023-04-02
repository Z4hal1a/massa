use massa_models::operation::{
    OperationPrefixIds, OperationPrefixIdsDeserializer, OperationPrefixIdsSerializer,
    OperationsDeserializer, OperationsSerializer, SecureShareOperation,
};
use massa_serialization::{Deserializer, SerializeError, Serializer, U64VarIntSerializer};
use nom::{
    error::{context, ContextError, ParseError},
    IResult, Parser,
};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug)]
pub enum OperationMessage {
    /// Batch of operation ids
    OperationsAnnouncement(OperationPrefixIds),
    /// Someone ask for operations.
    AskForOperations(OperationPrefixIds),
    /// A list of operations
    Operations(Vec<SecureShareOperation>),
}

#[derive(IntoPrimitive, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u64)]
enum MessageTypeId {
    OperationsAnnouncement = 0,
    AskForOperations = 1,
    Operations = 2,
}

#[derive(Default)]
pub struct OperationMessageSerializer {
    id_serializer: U64VarIntSerializer,
    operation_prefix_ids_serializer: OperationPrefixIdsSerializer,
    operations_serializer: OperationsSerializer,
}

impl OperationMessageSerializer {
    pub fn new() -> Self {
        Self {
            id_serializer: U64VarIntSerializer::new(),
            operation_prefix_ids_serializer: OperationPrefixIdsSerializer::new(),
            operations_serializer: OperationsSerializer::new(),
        }
    }
}

impl Serializer<OperationMessage> for OperationMessageSerializer {
    fn serialize(
        &self,
        value: &OperationMessage,
        buffer: &mut Vec<u8>,
    ) -> Result<(), SerializeError> {
        match value {
            OperationMessage::OperationsAnnouncement(operations) => {
                self.id_serializer
                    .serialize(&(MessageTypeId::OperationsAnnouncement as u64), buffer)?;
                self.operation_prefix_ids_serializer
                    .serialize(operations, buffer)?;
            }
            OperationMessage::AskForOperations(operations) => {
                self.id_serializer
                    .serialize(&(MessageTypeId::AskForOperations as u64), buffer)?;
                self.operation_prefix_ids_serializer
                    .serialize(operations, buffer)?;
            }
            OperationMessage::Operations(operations) => {
                self.id_serializer
                    .serialize(&(MessageTypeId::Operations as u64), buffer)?;
                self.operations_serializer.serialize(operations, buffer)?;
            }
        }
        Ok(())
    }
}

pub struct OperationMessageDeserializer {
    operation_prefix_ids_deserializer: OperationPrefixIdsDeserializer,
    operations_deserializer: OperationsDeserializer,
    message_id: u64,
}

/// Limits used in the deserialization of `OperationMessage`
pub struct OperationMessageDeserializerArgs {
    /// Maximum number of prefix ids that can be asked to propagate or sent
    pub max_operations_prefix_ids: u32,
    /// Maximum of full operations sent in one message
    pub max_operations: u32,
    //TODO: All of this arguments should be in a `OperationDeserializer` struct that would be used here
    ///
    pub max_datastore_value_length: u64,
    ///
    pub max_function_name_length: u16,
    ///
    pub max_parameters_size: u32,
    ///
    pub max_op_datastore_entry_count: u64,
    ///
    pub max_op_datastore_key_length: u8,
    ///
    pub max_op_datastore_value_length: u64,
}

impl OperationMessageDeserializer {
    pub fn new(args: OperationMessageDeserializerArgs) -> Self {
        Self {
            operation_prefix_ids_deserializer: OperationPrefixIdsDeserializer::new(
                args.max_operations_prefix_ids,
            ),
            operations_deserializer: OperationsDeserializer::new(
                args.max_operations,
                args.max_datastore_value_length,
                args.max_function_name_length,
                args.max_parameters_size,
                args.max_op_datastore_entry_count,
                args.max_op_datastore_key_length,
                args.max_op_datastore_value_length,
            ),
            message_id: 0,
        }
    }

    pub fn set_message_id(&mut self, id: u64) {
        self.message_id = id;
    }
}

impl Deserializer<OperationMessage> for OperationMessageDeserializer {
    fn deserialize<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        &self,
        buffer: &'a [u8],
    ) -> IResult<&'a [u8], OperationMessage, E> {
        context("Failed OperationMessage deserialization", |buffer| {
            let id = MessageTypeId::try_from(self.message_id).map_err(|_| {
                nom::Err::Error(ParseError::from_error_kind(
                    buffer,
                    nom::error::ErrorKind::Eof,
                ))
            })?;
            match id {
                MessageTypeId::AskForOperations => {
                    context("Failed AskForOperations deserialization", |input| {
                        self.operation_prefix_ids_deserializer.deserialize(input)
                    })
                    .map(OperationMessage::AskForOperations)
                    .parse(buffer)
                }
                MessageTypeId::OperationsAnnouncement => {
                    context("Failed OperationsAnnouncement deserialization", |input| {
                        self.operation_prefix_ids_deserializer.deserialize(input)
                    })
                    .map(OperationMessage::OperationsAnnouncement)
                    .parse(buffer)
                }
                MessageTypeId::Operations => {
                    context("Failed Operations deserialization", |input| {
                        self.operations_deserializer.deserialize(input)
                    })
                    .map(OperationMessage::Operations)
                    .parse(buffer)
                }
            }
        })
        .parse(buffer)
    }
}
