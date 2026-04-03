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

        // Validate bounds
        if start > end {
            return Err("Start index cannot be greater than end index".into());
        }
        let df_height = df.height();
        if start >= df_height {
            return Ok(Vec::new()); // Return empty if start is beyond data
        }

        // Clamp end to dataframe bounds
        let end = std::cmp::min(end, df_height);
        let length = end - start;

        let slice = df.slice(start as i64, length);
        let mut rows = Vec::new();

        // Cache column references for performance
        let columns: Vec<_> = slice
            .get_column_names()
            .iter()
            .map(|name| slice.column(name).unwrap())
            .collect();

        for row_idx in 0..slice.height() {
            let mut values = Vec::new();
            for column in &columns {
                let value = match column.get(row_idx)? {
                    AnyValue::Null => serde_json::Value::Null,
                    AnyValue::Boolean(v) => serde_json::Value::Bool(v),
                    AnyValue::Int8(v) => serde_json::Value::Number(serde_json::Number::from(v)),
                    AnyValue::Int16(v) => serde_json::Value::Number(serde_json::Number::from(v)),
                    AnyValue::Int32(v) => serde_json::Value::Number(serde_json::Number::from(v)),
                    AnyValue::Int64(v) => serde_json::Value::Number(serde_json::Number::from(v)),
                    AnyValue::UInt8(v) => serde_json::Value::Number(serde_json::Number::from(v)),
                    AnyValue::UInt16(v) => serde_json::Value::Number(serde_json::Number::from(v)),
                    AnyValue::UInt32(v) => serde_json::Value::Number(serde_json::Number::from(v)),
                    AnyValue::UInt64(v) => serde_json::Value::Number(serde_json::Number::from(v)),
                    AnyValue::Float32(v) => {
                        if v.is_finite() {
                            serde_json::Value::Number(
                                serde_json::Number::from_f64(v as f64)
                                    .unwrap_or(serde_json::Number::from(0)),
                            )
                        } else {
                            serde_json::Value::String(v.to_string())
                        }
                    }
                    AnyValue::Float64(v) => {
                        if v.is_finite() {
                            serde_json::Value::Number(
                                serde_json::Number::from_f64(v)
                                    .unwrap_or(serde_json::Number::from(0)),
                            )
                        } else {
                            serde_json::Value::String(v.to_string())
                        }
                    }
                    AnyValue::String(v) => serde_json::Value::String(v.to_string()),
                    AnyValue::Date(v) => serde_json::Value::String(v.to_string()),
                    AnyValue::Datetime(v, unit, tz) => {
                        serde_json::Value::String(format!("{:?} {:?} {:?}", v, unit, tz))
                    }
                    AnyValue::Duration(v, unit) => {
                        serde_json::Value::String(format!("{:?} {:?}", v, unit))
                    }
                    AnyValue::Time(v) => serde_json::Value::String(v.to_string()),
                    AnyValue::List(v) => serde_json::Value::Array(
                        v.iter()
                            .map(|val| match val {
                                AnyValue::Null => serde_json::Value::Null,
                                AnyValue::Boolean(b) => serde_json::Value::Bool(b),
                                AnyValue::Int8(i) => {
                                    serde_json::Value::Number(serde_json::Number::from(i))
                                }
                                AnyValue::Int16(i) => {
                                    serde_json::Value::Number(serde_json::Number::from(i))
                                }
                                AnyValue::Int32(i) => {
                                    serde_json::Value::Number(serde_json::Number::from(i))
                                }
                                AnyValue::Int64(i) => {
                                    serde_json::Value::Number(serde_json::Number::from(i))
                                }
                                AnyValue::UInt8(i) => {
                                    serde_json::Value::Number(serde_json::Number::from(i))
                                }
                                AnyValue::UInt16(i) => {
                                    serde_json::Value::Number(serde_json::Number::from(i))
                                }
                                AnyValue::UInt32(i) => {
                                    serde_json::Value::Number(serde_json::Number::from(i))
                                }
                                AnyValue::UInt64(i) => {
                                    serde_json::Value::Number(serde_json::Number::from(i))
                                }
                                AnyValue::Float32(f) => serde_json::Value::Number(
                                    serde_json::Number::from_f64(f as f64)
                                        .unwrap_or(serde_json::Number::from(0)),
                                ),
                                AnyValue::Float64(f) => serde_json::Value::Number(
                                    serde_json::Number::from_f64(f)
                                        .unwrap_or(serde_json::Number::from(0)),
                                ),
                                AnyValue::String(s) => serde_json::Value::String(s.to_string()),
                                other => serde_json::Value::String(other.to_string()),
                            })
                            .collect(),
                    ),
                    AnyValue::Struct(idx, _arr, fields) => serde_json::Value::String(format!(
                        "Struct(idx: {}, fields: {:?})",
                        idx,
                        fields.len()
                    )),
                    AnyValue::Binary(v) => {
                        serde_json::Value::String(format!("Binary({} bytes)", v.len()))
                    }
                    other => serde_json::Value::String(other.to_string()),
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
