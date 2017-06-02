use bincode;
use serde_json;
use rmp_serde;
use serde::Serialize;

/// Перечисление для хранения типа формата сериализации.
#[derive(Debug, Eq, PartialEq)]
pub enum SerializationFormat {
    /// JSON
    Json,
    /// MsgPack
    MsgPack,
    /// Bincode
    Bincode,
}

impl SerializationFormat {
    /// Сериализует переданное значение в зависимости от типа выбранного формата сериализации.
    pub fn serialize<SerializableType>(&self, value: &SerializableType) -> Vec<u8>
        where SerializableType: Serialize
    {
        match self {
            &SerializationFormat::Json => serde_json::to_string(&value).unwrap().into_bytes(),
            &SerializationFormat::MsgPack => rmp_serde::to_vec(&value).unwrap(),
            &SerializationFormat::Bincode => bincode::serialize(&value, bincode::Infinite).unwrap(),
        }
    }
}
