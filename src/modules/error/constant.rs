use super::custom::CustomErrorType;

pub const INVALID_ID_CODE: (u16, &str, CustomErrorType) = (
    4001,
    "Invalid format identifier",
    CustomErrorType::DieselError,
);

pub const INVALID_PAYLOAD: (u16, &str, CustomErrorType) =
    (4002, "Invalid payload", CustomErrorType::ValidationError);

pub const INVALID_AUTHORIZATION_HEADER: (u16, &str, CustomErrorType) = (
    4003,
    "Invalid authorization param header",
    CustomErrorType::InvalidAuthorizationHeader,
);
