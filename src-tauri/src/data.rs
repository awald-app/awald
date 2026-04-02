use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use tauri::State;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnSchema {
    pub name: String,
    pub dtype: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetMeta {
    pub id: String,
    pub filename: String,
    pub nrows: usize,
    pub ncols: usize,
    pub schema: Vec<ColumnSchema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RowData {
    pub values: Vec<serde_json::Value>,
}

#[derive(Debug)]
pub struct DataStore {
    pub datasets: Arc<RwLock<std::collections::HashMap<String, DataFrame>>>,
}

impl DataStore {
    pub fn new() -> Self {
        Self {
            datasets: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    pub fn load_csv(
        &self,
        path: &str,
    ) -> Result<(DataFrame, DatasetMeta), Box<dyn std::error::Error>> {
        let df = CsvReader::new(std::fs::File::open(path)?).finish()?;

        let filename = std::path::Path::new(path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let schema: Vec<ColumnSchema> = df
            .get_column_names()
            .iter()
            .zip(df.dtypes().iter())
            .map(|(name, dtype)| ColumnSchema {
                name: name.as_str().to_string(),
                dtype: dtype.to_string(),
            })
            .collect();

        let meta = DatasetMeta {
            id: Uuid::new_v4().to_string(),
            filename,
            nrows: df.height(),
            ncols: df.width(),
            schema,
        };

        Ok((df, meta))
    }

    pub fn store_dataset(&self, id: &str, df: DataFrame) -> Result<(), Box<dyn std::error::Error>> {
        let mut datasets = self
            .datasets
            .write()
            .map_err(|e| format!("Lock error: {}", e))?;
        datasets.insert(id.to_string(), df);
        Ok(())
    }

    pub fn get_rows(
        &self,
        id: &str,
        start: usize,
        end: usize,
    ) -> Result<Vec<RowData>, Box<dyn std::error::Error>> {
        let datasets = self
            .datasets
            .read()
            .map_err(|e| format!("Lock error: {}", e))?;
        let df = datasets.get(id).ok_or("Dataset not found")?;

        let slice = df.slice(start as i64, end - start);
        let mut rows = Vec::new();

        for row_idx in 0..slice.height() {
            let mut values = Vec::new();
            for col_idx in 0..slice.width() {
                let column_name = slice.get_column_names()[col_idx];
                let column = slice.column(column_name)?;
                let value = match column.get(row_idx)? {
                    AnyValue::Null => serde_json::Value::Null,
                    AnyValue::Boolean(v) => serde_json::Value::Bool(v),
                    AnyValue::Int64(v) => serde_json::Value::Number(serde_json::Number::from(v)),
                    AnyValue::Float64(v) => serde_json::Value::Number(
                        serde_json::Number::from_f64(v).unwrap_or(serde_json::Number::from(0)),
                    ),
                    AnyValue::String(v) => serde_json::Value::String(v.to_string()),
                    AnyValue::UInt64(v) => serde_json::Value::Number(serde_json::Number::from(v)),
                    _ => serde_json::Value::String("unsupported".to_string()),
                };
                values.push(value);
            }
            rows.push(RowData { values });
        }

        Ok(rows)
    }
}

#[tauri::command]
pub async fn load_file(path: String, store: State<'_, DataStore>) -> Result<DatasetMeta, String> {
    let (df, meta) = store.load_csv(&path).map_err(|e| e.to_string())?;
    let id = meta.id.clone();

    store.store_dataset(&id, df).map_err(|e| e.to_string())?;

    Ok(meta)
}

#[tauri::command]
pub async fn get_rows(
    id: String,
    start: usize,
    end: usize,
    store: State<'_, DataStore>,
) -> Result<Vec<RowData>, String> {
    store.get_rows(&id, start, end).map_err(|e| e.to_string())
}
