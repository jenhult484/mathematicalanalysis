use std::collections::HashMap;

fn main() {
    let mut map: HashMap<String, i32> = HashMap::new();
    
    // Example data structure mapping keys to values
    let data = vec![
        ("key1", 5),
        ("key2", -10),
        ("key3", 8.75),
        ("key4", "test"),
        ("key5", true),
        ("key6", 9.81),
        ("key7", 'A'),
        ("key8", 2.72),
    ];

    for (key, value) in data {
        map.insert(key.clone(), value);
    }

    // Example usage
    println!("The values are: {:?}", map.values());
}

