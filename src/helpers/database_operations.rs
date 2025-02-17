/// DuckDB storage operations implementation
/// 
/// Provides functionality for:
/// - Data persistence
/// - Record updates
/// - Batch operations
use duckdb::{Connection, Result, ToSql};
use serde_json::Value;

pub fn save_records(conn: &Connection, data: &[Value], table_name: &str) -> Result<()> {
    for record in data {
        let obj = record.as_object().unwrap();
        let columns = obj.keys().map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
        let placeholders = (0..obj.len())
            .map(|i| format!("${}", i + 1))
            .collect::<Vec<_>>()
            .join(", ");
        
        let sql = format!(
            "INSERT OR REPLACE INTO {} ({}) VALUES ({})",
            table_name,
            columns,
            placeholders
        );
        
        let values: Vec<String> = obj.values()
            .map(|v| v.to_string())
            .collect();
        
        // Convert values to a slice of references that implement ToSql
        let param_refs: Vec<&(dyn ToSql)> = values.iter()
            .map(|s| s as &(dyn ToSql))
            .collect();
        
        conn.execute(&sql, param_refs.as_slice())?;
    }
    
    Ok(())
} 