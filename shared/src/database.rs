//! Database connection and management utilities

use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};
use std::time::Duration;
use tracing::{info, instrument, warn};

/// Database connection pool manager
#[derive(Clone)]
pub struct DatabaseManager {
    pool: Pool<Postgres>,
}

impl DatabaseManager {
    /// Create a new database manager with connection pool
    #[instrument(skip(database_url))]
    pub async fn new(database_url: &str) -> Result<Self> {
        info!("Initializing database connection pool");
        
        let pool = PgPoolOptions::new()
            .max_connections(20)
            .min_connections(5)
            .acquire_timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(600))
            .max_lifetime(Duration::from_secs(1800))
            .test_before_acquire(true)
            .connect(database_url)
            .await?;

        // Test the connection
        let row: (i64,) = sqlx::query_as("SELECT 1")
            .fetch_one(&pool)
            .await?;
        
        if row.0 != 1 {
            anyhow::bail!("Database connection test failed");
        }

        info!("Database connection pool initialized successfully");
        
        Ok(Self { pool })
    }

    /// Get a reference to the connection pool
    pub fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }

    /// Check database health
    #[instrument(skip(self))]
    pub async fn health_check(&self) -> bool {
        match sqlx::query("SELECT 1").fetch_one(&self.pool).await {
            Ok(_) => true,
            Err(e) => {
                warn!("Database health check failed: {}", e);
                false
            }
        }
    }

    /// Run database migrations
    #[instrument(skip(self))]
    pub async fn migrate(&self) -> Result<()> {
        info!("Running database migrations");
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        info!("Database migrations completed successfully");
        Ok(())
    }

    /// Get database statistics
    #[instrument(skip(self))]
    pub async fn get_stats(&self) -> Result<DatabaseStats> {
        let row = sqlx::query(
            r#"
            SELECT 
                numbackends as active_connections,
                xact_commit as transactions_committed,
                xact_rollback as transactions_rolled_back,
                blks_read as blocks_read,
                blks_hit as blocks_hit,
                tup_returned as tuples_returned,
                tup_fetched as tuples_fetched,
                tup_inserted as tuples_inserted,
                tup_updated as tuples_updated,
                tup_deleted as tuples_deleted
            FROM pg_stat_database 
            WHERE datname = current_database()
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(DatabaseStats {
            active_connections: row.get("active_connections"),
            transactions_committed: row.get("transactions_committed"),
            transactions_rolled_back: row.get("transactions_rolled_back"),
            blocks_read: row.get("blocks_read"),
            blocks_hit: row.get("blocks_hit"),
            tuples_returned: row.get("tuples_returned"),
            tuples_fetched: row.get("tuples_fetched"),
            tuples_inserted: row.get("tuples_inserted"),
            tuples_updated: row.get("tuples_updated"),
            tuples_deleted: row.get("tuples_deleted"),
        })
    }
}

/// Database performance statistics
#[derive(Debug, Clone)]
pub struct DatabaseStats {
    pub active_connections: i32,
    pub transactions_committed: i64,
    pub transactions_rolled_back: i64,
    pub blocks_read: i64,
    pub blocks_hit: i64,
    pub tuples_returned: i64,
    pub tuples_fetched: i64,
    pub tuples_inserted: i64,
    pub tuples_updated: i64,
    pub tuples_deleted: i64,
}

impl DatabaseStats {
    /// Calculate cache hit ratio
    pub fn cache_hit_ratio(&self) -> f64 {
        if self.blocks_read + self.blocks_hit == 0 {
            0.0
        } else {
            self.blocks_hit as f64 / (self.blocks_read + self.blocks_hit) as f64
        }
    }

    /// Calculate transaction success ratio
    pub fn transaction_success_ratio(&self) -> f64 {
        let total = self.transactions_committed + self.transactions_rolled_back;
        if total == 0 {
            1.0
        } else {
            self.transactions_committed as f64 / total as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_stats_calculations() {
        let stats = DatabaseStats {
            active_connections: 5,
            transactions_committed: 100,
            transactions_rolled_back: 5,
            blocks_read: 20,
            blocks_hit: 80,
            tuples_returned: 1000,
            tuples_fetched: 500,
            tuples_inserted: 100,
            tuples_updated: 50,
            tuples_deleted: 10,
        };

        assert_eq!(stats.cache_hit_ratio(), 0.8);
        assert!((stats.transaction_success_ratio() - 0.952).abs() < 0.001);
    }

    #[test]
    fn test_zero_division_safety() {
        let stats = DatabaseStats {
            active_connections: 0,
            transactions_committed: 0,
            transactions_rolled_back: 0,
            blocks_read: 0,
            blocks_hit: 0,
            tuples_returned: 0,
            tuples_fetched: 0,
            tuples_inserted: 0,
            tuples_updated: 0,
            tuples_deleted: 0,
        };

        assert_eq!(stats.cache_hit_ratio(), 0.0);
        assert_eq!(stats.transaction_success_ratio(), 1.0);
    }
}