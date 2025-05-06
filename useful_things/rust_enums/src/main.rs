use serde::Serialize;

// internally tagged enums
#[derive(Serialize)]
#[serde(tag = "outcome")]
enum Response1 {
    Success { value: String },
    Failure { error: String },
}

// externally tagged enums
#[derive(Serialize)]
enum Response2 {
    Success { value: String },
    Failure { error: String },
}

// adjacently tagged enums
#[derive(Serialize)]
#[serde(tag = "outcome", content = "content")]
enum Response3 {
    Success { value: String },
    Failure { error: String },
}

// untagged enums
#[derive(Serialize)]
#[serde(untagged)]
enum Response4 {
    Success { value: String },
    Failure { error: String },
}

fn main() {
    // externally tagged enums
    let message = Response1::Success {
        value: String::from("result"),
    };
    let serialized = serde_json::to_string(&message).unwrap();
    println!("serialized = {}", serialized);

    // internally tagged enums
    let message = Response2::Success {
        value: String::from("result"),
    };
    let serialized = serde_json::to_string(&message).unwrap();
    println!("serialized = {}", serialized);

    // adjacently tagged enums
    let message = Response3::Success {
        value: String::from("result"),
    };
    let serialized = serde_json::to_string(&message).unwrap();
    println!("serialized = {}", serialized);

    // untagged enums
    let message = Response4::Success {
        value: String::from("result"),
    };
    let serialized = serde_json::to_string(&message).unwrap();
    println!("serialized = {}", serialized);
}
