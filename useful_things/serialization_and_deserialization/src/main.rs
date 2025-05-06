use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Message {
    content: String,
}

#[derive(Serialize)]
struct Message2 {
    n1: i64,
    n2: f64,
    b: bool,
    c: char,
    s: String,
}

#[derive(Serialize)]
struct Message3 {
    numbers: [i64; 5],
    strings: Vec<String>,
    tuple: (i64, String),
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Message4 {
    one_field: i64,
    other_field: i64,
    #[serde(rename = "exception")]
    and_an_exception: i64,
}

fn main() {
    let message = Message {
        content: String::from("something"),
    };

    let serialized = serde_json::to_string(&message).unwrap();
    println!("Serialized: {}", serialized);

    let deserialized: Message = serde_json::from_str(&serialized).unwrap();
    println!("deserialized content = {}", deserialized.content);

    let message2 = Message2 {
        n1: 42,
        n2: 3.14,
        b: true,
        c: 'c',
        s: String::from("hello"),
    };

    let serialized2 = serde_json::to_string(&message2).unwrap();
    println!("serialized = {}", serialized2);

    let message3 = Message3 {
        numbers: [1, 2, 3, 4, 5],
        strings: vec![String::from("one"), String::from("two")],
        tuple: (42, String::from("something")),
    };

    let serialized3 = serde_json::to_string(&message3).unwrap();
    println!("serialized3 = {}", serialized3);

    let message4 = Message4 {
        one_field: 1,
        other_field: 2,
        and_an_exception: 3,
    };

    let serialized4 = serde_json::to_string(&message4).unwrap();
    println!("serialized4 = {}", serialized4);
}
