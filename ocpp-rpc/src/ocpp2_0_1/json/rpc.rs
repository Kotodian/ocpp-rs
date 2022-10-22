use std::str::FromStr;

use serde_json::Value;

use super::enums::{Action, MessageType, Payload};

type MessageTypeId = usize;
type MessageId = String;
type ErrorCode = String;
type ErrorDescription = String;
type ErrorDetails = Value;

#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Call {
    pub message_type_id: MessageTypeId,
    pub message_id: MessageId,
    pub action: Action,
    pub payload: Payload,
}

impl TryFrom<MessageType> for Call {
    type Error = &'static str;

    fn try_from(msg: MessageType) -> Result<Self, Self::Error> {
        match msg {
            MessageType::Call(message_type_id, message_id, action, payload) => {
                let action = if let Ok(o) = Action::from_str(&action) {
                    o
                } else {
                    return Err("failed to find action");
                };
                let payload: Payload = if let Ok(p) = serde_json::from_value::<Payload>(payload) {
                    p
                } else {
                    return Err("failed to parse payload");
                };
                Ok(Call {
                    message_type_id,
                    message_id,
                    action,
                    payload,
                })
            }
            _ => Err("failed to parse to call"),
        }
    }
}

impl serde::Serialize for Call {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let t = (
            &self.message_type_id,
            &self.message_id,
            &self.action.to_string(),
            &self.payload,
        );
        t.serialize(serializer)
    }
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CallResult {
    pub message_type_id: MessageTypeId,
    pub message_id: MessageId,
    pub payload: Payload,
}

impl TryFrom<MessageType> for CallResult {
    type Error = &'static str;

    fn try_from(msg: MessageType) -> Result<Self, Self::Error> {
        match msg {
            MessageType::CallResult(message_type_id, message_id, payload) => {
                let payload: Payload =
                    if let Ok(p) = serde_json::from_value::<Payload>(payload) {
                        p
                    } else {
                        return Err("failed");
                    };
                Ok(CallResult {
                    message_type_id,
                    message_id,
                    payload,
                })
            }
            _ => Err("failed"),
        }
    }
}

impl serde::Serialize for CallResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (&self.message_type_id, &self.message_id, &self.payload).serialize(serializer)
    }
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
/// CallError: [<MessageTypeId>, "<MessageId>", "<errorCode>", "<errorDescription>", {<errorDetails>}]
pub struct CallError {
    pub message_type_id: MessageTypeId,
    pub message_id: MessageId,
    pub error_code: ErrorCode,
    pub error_description: ErrorDescription,
    pub error_details: ErrorDetails,
}

impl TryFrom<MessageType> for CallError {
    type Error = &'static str;

    fn try_from(msg: MessageType) -> Result<Self, Self::Error> {
        match msg {
            MessageType::CallError(
                message_type_id,
                message_id,
                error_code,
                error_description,
                error_details,
            ) => Ok(CallError {
                message_type_id,
                message_id,
                error_code,
                error_description,
                error_details,
            }),
            _ => Err("failed"),
        }
    }
}

impl serde::Serialize for CallError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (
            &self.message_type_id,
            &self.message_id,
            &self.error_code,
            &self.error_description,
            &self.error_details,
        )
            .serialize(serializer)
    }
}
