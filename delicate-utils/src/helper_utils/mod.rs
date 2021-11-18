pub mod byte_buf;

pub fn get_unique_id_string() -> String {
    ::uuid::Uuid::new_v4().to_hyphenated().to_string()
}
