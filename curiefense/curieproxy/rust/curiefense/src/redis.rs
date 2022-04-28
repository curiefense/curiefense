use crate::{
    interface::{SimpleAction, SimpleActionT},
    logs::Logs,
};
use lazy_static::lazy_static;
use redis::{ConnectionAddr, ConnectionInfo, RedisConnectionInfo};

lazy_static! {
    static ref RPOOL: anyhow::Result<redis::aio::ConnectionManager> = async_std::task::block_on(build_pool());
}

/// creates an async connection to a redis server
pub async fn build_pool() -> anyhow::Result<redis::aio::ConnectionManager> {
    let server = std::env::var("REDIS_HOST").unwrap_or_else(|_| "redis".to_string());
    let port = std::env::var("REDIS_PORT").unwrap_or_else(|_| "6379".to_string());
    let db = std::env::var("REDIS_DB").unwrap_or_else(|_| "0".to_string());
    let username = std::env::var("REDIS_USERNAME").ok();
    let password = std::env::var("REDIS_PASSWORD").ok();
    let addr = ConnectionAddr::Tcp(server, port.parse()?);
    let redis = RedisConnectionInfo {
        db: db.parse()?,
        username,
        password,
    };
    let cinfo = ConnectionInfo { addr, redis };
    let client = redis::Client::open(cinfo)?;
    let o = redis::aio::ConnectionManager::new(client).await?;
    Ok(o)
}

/// creates an async connection to a redis server
pub async fn redis_async_conn() -> anyhow::Result<redis::aio::ConnectionManager> {
    match &*RPOOL {
        Ok(c) => Ok(c.clone()),
        Err(rr) => Err(anyhow::anyhow!("{}", rr)),
    }
}

pub enum BanStatus {
    NewBan,
    AlreadyBanned,
}

pub async fn extract_bannable_action<CNX: redis::aio::ConnectionLike>(
    cnx: &mut CNX,
    logs: &mut Logs,
    action: &SimpleAction,
    redis_key: &str,
    ban_key: &str,
    ban_status: BanStatus,
) -> SimpleAction {
    if let SimpleActionT::Ban(subaction, duration) = &action.atype {
        logs.info(|| format!("Banned key {} for {}s", redis_key, duration));
        match ban_status {
            BanStatus::AlreadyBanned => (),
            BanStatus::NewBan => {
                if let Err(rr) = redis::pipe()
                    .cmd("SET")
                    .arg(ban_key)
                    .arg(1)
                    .cmd("EXPIRE")
                    .arg(ban_key)
                    .arg(*duration)
                    .query_async::<_, ()>(cnx)
                    .await
                {
                    println!("*** Redis error {}", rr);
                }
            }
        }
        *subaction.clone()
    } else {
        action.clone()
    }
}

pub fn get_ban_key(key: &str) -> String {
    format!("{:X}", md5::compute(format!("limit-ban-hash{}", key)))
}

pub async fn is_banned<CNX: redis::aio::ConnectionLike>(cnx: &mut CNX, ban_key: &str) -> bool {
    let q: redis::RedisResult<Option<u32>> = redis::cmd("GET").arg(ban_key).query_async(cnx).await;
    q.unwrap_or(None).is_some()
}
