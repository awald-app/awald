use pyo3::Python;
use pyo3::prelude::*;

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
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
