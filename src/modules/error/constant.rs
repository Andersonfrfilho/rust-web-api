use super::custom::CustomErrorType;

pub const INVALID_ID_CODE: (u16, &str, CustomErrorType) = (
    4001,
    "Invalid format identifier",
    CustomErrorType::DieselError,
);

pub const INVALID_PAYLOAD: (u16, &str, CustomErrorType) =
    (4002, "Invalid payload", CustomErrorType::ValidationError);

pub const MISSING_AUTHORIZATION_HEADER: (u16, &str, CustomErrorType) = (
    4003,
    "Missing authorization header parameter",
    CustomErrorType::InvalidAuthorizationHeader,
);

pub const ERROR_CONVERT_VALUE_TO_STRING: (u16, &str, CustomErrorType) = (
    4004,
    "Error convert value to string",
    CustomErrorType::ErrorConvertValueToString,
);

pub const HEADER_AUTHORIZATION_BEARER_INCOMPLETE: (u16, &str, CustomErrorType) = (
    4005,
    "Header authorization bearer incomplete",
    CustomErrorType::InvalidAuthorizationHeader,
);
