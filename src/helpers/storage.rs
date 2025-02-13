/// SQLite storage operations implementation
/// 
/// Provides functionality for:
/// - Data persistence
/// - Record updates
/// - Batch operations
use serde_json::Value;
use sqlx::{sqlite::SqlitePool};

pub async fn save_to_sqlite(pool: &SqlitePool, data: &[Value], table_name: &str) -> Result<(), sqlx::Error> {
    for record in data {
        let columns = record.as_object().unwrap().keys().map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
        let placeholders = (0..record.as_object().unwrap().len()).map(|i| format!("${}", i + 1)).collect::<Vec<_>>().join(", ");
        
        let sql = format!("INSERT OR REPLACE INTO {} ({}) VALUES ({})", table_name, columns, placeholders);
        
        let mut query = sqlx::query(&sql);
        for value in record.as_object().unwrap().values() {
            query = query.bind(value.as_str().unwrap_or(""));
        }
        
        query.execute(pool).await?;
    }
    
    Ok(())
}