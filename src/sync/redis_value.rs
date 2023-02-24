use std::time::SystemTime;

pub struct RedisValue {
    pub value: String,
    pub expire_at: Option<SystemTime>,
}
