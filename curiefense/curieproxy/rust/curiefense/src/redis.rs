use core::time::Duration;
use lazy_static::lazy_static;

use r2d2_redis::{r2d2, RedisConnectionManager};

lazy_static! {
    static ref RPOOL: anyhow::Result<r2d2::Pool<RedisConnectionManager>> = build_pool();
}

fn build_pool() -> anyhow::Result<r2d2::Pool<RedisConnectionManager>> {
    let server = std::env::var("REDIS_HOST").unwrap_or_else(|_| "redis".to_string());
    let manager = RedisConnectionManager::new(format!("redis://{}:6379/", server))?;
    let max_timeout = Duration::from_millis(100);
    let pool = r2d2::Pool::builder().connection_timeout(max_timeout).build(manager)?;
    Ok(pool)
}

pub type RedisCnx = r2d2::PooledConnection<RedisConnectionManager>;

/// creates a connection to a redis server
pub fn redis_conn() -> anyhow::Result<RedisCnx> {
    let pool = match &*RPOOL {
        Err(rr) => return Err(anyhow::anyhow!("{}", rr)),
        Ok(pl) => pl.clone(),
    };
    let cnx = pool.get()?;
    let max_timeout = Duration::from_millis(100);
    cnx.set_read_timeout(Some(max_timeout))?;
    cnx.set_write_timeout(Some(max_timeout))?;
    Ok(cnx)
}
