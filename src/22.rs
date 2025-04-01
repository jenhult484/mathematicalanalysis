fn main() {
    // Example of using `serde` library for serialization/deserialization.
    let serialized_data = serde_json::to_string(&json!({"name": "Alice", "age": 25}).unwrap())?;
    println!("Serialized data: {}", serialized_data);
    
    // Deserialize the JSON string back to a struct.
    let deserialized_data: serde_json::Value = serde_json::from_str(&serialized_data)?;
    println!("Deserialized data: {:?}", deserialized_data);
}
