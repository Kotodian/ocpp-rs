pub enum ErrorCodes {
    FormatViolation,
    GenericError,
    InternalError,
    MessageTypeNotSupported,
    NotImplemented,
    NotSupported,
    OccurrenceConstraintViolation,
    PropertyConstraintViolation,
    ProtocolError,
    SecurityError,
    TypeConstraintViolation,
}

impl ErrorCodes {
    pub fn description(&self) -> &str {
        match self {
        ErrorCodes::FormatViolation => "Payload for Action is syntactically incorrect",
             ErrorCodes::GenericError => "Any other error not covered by the more specific error codes in this table",
            ErrorCodes::InternalError => "An internal error occurred and the receiver was not able to process the requested Action successfully" ,
            ErrorCodes::MessageTypeNotSupported => "A message with an Message Type Number received that is not supported by this implementation.",
            ErrorCodes::NotImplemented => "Requested Action is not known by receiver",
            ErrorCodes::NotSupported => "Requested Action is recognized but not supported by the receiver",
            ErrorCodes::OccurrenceConstraintViolation => "Payload for Action is syntactically correct but at least one of the fields violates occurrence constraints",
            ErrorCodes::PropertyConstraintViolation => "Payload is syntactically correct but at least one field contains an invalid value",
            ErrorCodes::ProtocolError => "Payload for Action is not conform the PDU structure",
            ErrorCodes::SecurityError => "During the processing of Action a security issue occurred preventing receiver from completing the Action successfully",
            ErrorCodes::TypeConstraintViolation => "Payload for Action is syntactically correct but at least one of the fields violates data type constraints (e.g. \"somestring\": 12)",
        }
    }
}
