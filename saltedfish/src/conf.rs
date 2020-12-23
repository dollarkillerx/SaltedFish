use serde::{Deserialize, Serialize};
use std::sync::Arc;
use once_cell::sync::OnceCell;

async fn global_conf() -> &'static Conf {
    static CONF: OnceCell<Conf> = OnceCell::new();
    CONF.get_or_init(|| {
        Conf{
            path: "xxx",
            query_concurrency: 100,
            block_size: 25,
        }
    })
}

/// Configuration file 配置文件
#[derive(Deserialize, Serialize, Debug)]
pub struct Conf {
    /// Storage Path 存储路径
    #[serde(rename = "Path")]
    pub path: &'static str,
    /// Limit the number of concurrent queries 查询并发数量限制
    #[serde(rename = "QueryConcurrency")]
    pub query_concurrency: u32,
    /// Data Block Size 数据区块大小MB
    #[serde(rename = "BlockSize")]
    pub block_size: u64,
}