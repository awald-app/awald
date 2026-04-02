mod data;

use pyo3::prelude::*;
use pyo3::Python;

use data::{get_rows, load_file, DataStore};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // pyo3 smoke test
    Python::with_gil(|py| {
        let result = py.eval_bound("1+1", None, None).unwrap();
        let value: i32 = result.extract().unwrap();
        println!("Python smoke test result: {}", value);
        assert_eq!(value, 2);
    });

    tauri::Builder::default()
        .manage(DataStore::new())
        .invoke_handler(tauri::generate_handler![load_file, get_rows])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
                app.handle().plugin(tauri_plugin_dialog::init())?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
