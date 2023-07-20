use super::custom::CustomErrorType;

pub const INVALID_ID_CODE: (u16, &str, CustomErrorType) = (
    4001,
    "Invalid format identifier",
    CustomErrorType::DieselError,
);
