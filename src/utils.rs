use actix_web::http::header::{HeaderMap, HeaderValue};

pub fn obfuscator_part_of_value(header_value: Option<&HeaderValue>) -> String {
    let value_string = match header_value {
        None => panic!("value not defined"),
        Some(header) => header.to_str().unwrap(),
    };

    if value_string.len() >= 9 {
        let obfuscated_value = format!(
            "{}***{}",
            value_string[0..3].to_string(),
            value_string[value_string.len() - 3..value_string.len()].to_string()
        );
        return obfuscated_value;
    }

    return "***".to_string();
}

pub fn view_header(header_value: &HeaderMap) -> String {
    println!("###### {:?}", header_value);

    return "***".to_string();
}
