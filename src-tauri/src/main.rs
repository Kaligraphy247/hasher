// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// https://github.com/tauri-apps/tauri/discussions/5367

mod hash_functions;
use hash_functions::hashers::{sha2_256, sha3_256};
use serde;
use std::{collections::BTreeMap, sync::Mutex};
use tauri::{Manager, State};

#[allow(dead_code)]
#[derive(Debug)]
struct Counter {
    count: Mutex<i32>,
}

/// Structure for sending and receiving events.
///
/// It is the equivalent for `hash_text` from the JS end & `hash_result` from the Rust end.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct HashText {
    text: String,
    function_type: String,
}

/// Represents the available functions to be called in the `hash_text` event
#[derive(Debug, Eq, Hash, PartialEq)]
struct FunctionMap {
    label: String,
    value: fn(&[u8]) -> String,
}

impl FunctionMap {
    fn new(label: String, value: fn(&[u8]) -> String) -> FunctionMap {
        FunctionMap { label, value }
    }
}

fn main() {
    // Fix path env for tauri apps on MacOS & Linux
    fix_path_env::fix().expect("Failed to fix path environment variable");
    let available_functions: Vec<FunctionMap> = Vec::from([
        FunctionMap::new("sha2_256".into(), sha2_256),
        FunctionMap::new("sha3_256".into(), sha3_256),
    ]);

    let mut functions_map = BTreeMap::new();
    for function in available_functions {
        functions_map.insert(function.label, function.value);
    }

    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            app.manage(Counter {
                count: Mutex::new(0),
            });

            // let counter = handle.state::<Counter>();
            // *counter.count.lock().unwrap() += 1;
            // println!("COUNT: {:?}", *counter.count.lock().unwrap());

            let _id = app.listen_global("hash_text", move |event_handler| {
                let event_payload: HashText =
                    serde_json::from_str(event_handler.payload().unwrap()).unwrap();

                println!("Got event from JS\nPayload: {:?}\n", event_payload);
                let hash_result = functions_map.get(&event_payload.function_type).unwrap()(
                    &event_payload.text.as_bytes(),
                );

                println!("hash_result: {hash_result:?}\n",); // hash_result
                                                             // Emit result event to JS
                handle
                    .emit_all(
                        "hash_result",
                        HashText {
                            text: hash_result,
                            function_type: event_payload.function_type,
                        },
                    )
                    .unwrap();
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            sha2_256,
            sha3_256,
            get_count,
            increment_count,
            decrement_count
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_count(counter: State<Counter>) -> i32 {
    println!("⦁ Count Value - GET: {:?}", *counter.count.lock().unwrap());
    *counter.count.lock().unwrap()
}

#[tauri::command]
fn increment_count(counter: State<Counter>) -> i32 {
    println!("+ Count Value Before: {:?}", *counter.count.lock().unwrap());
    *counter.count.lock().unwrap() += 8;
    println!("+ Count Value After: {:?}", *counter.count.lock().unwrap());
    *counter.count.lock().unwrap()
}

#[tauri::command]
fn decrement_count(counter: State<Counter>) -> i32 {
    println!("- Count Value Before: {:?}", *counter.count.lock().unwrap());
    *counter.count.lock().unwrap() -= 8;
    println!("- Count Value After: {:?}", *counter.count.lock().unwrap());
    *counter.count.lock().unwrap()
}
