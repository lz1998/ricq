use std::net::SocketAddr;
use std::time::Duration;

use tokio::net::TcpStream;
use tokio::task::JoinSet;

pub async fn tcp_connect_timeout(
    addr: SocketAddr,
    timeout: Duration,
) -> tokio::io::Result<TcpStream> {
    let conn = tokio::net::TcpStream::connect(addr);
    tokio::time::timeout(timeout, conn)
        .await
        .map_err(tokio::io::Error::from)
        .flatten()
}

/// Race the given address, call `join_set.join_next()` to get next fastest `(addr, conn)` pair.
async fn race_addrs(
    addrs: Vec<SocketAddr>,
    timeout: Duration,
) -> JoinSet<tokio::io::Result<(SocketAddr, TcpStream)>> {
    let mut join_set = JoinSet::new();
    for addr in addrs {
        join_set.spawn(async move {
            let a = addr;
            tcp_connect_timeout(addr, timeout).await.map(|s| (a, s))
        });
    }
    join_set
}

pub async fn sort_addrs<Addr>(addrs: Vec<Addr>, timeout: Duration) -> Vec<Addr>
where
    SocketAddr: From<Addr>,
    Addr: From<SocketAddr>,
{
    let mut join_set = race_addrs(addrs.into_iter().map(Into::into).collect(), timeout).await;
    let mut ret = Vec::new();
    while let Some(result) = join_set.join_next().await {
        if let Ok(Ok((addr, _))) = result {
            ret.push(addr.into());
        }
    }
    ret
}

pub async fn tcp_connect_fastest(
    addrs: Vec<SocketAddr>,
    timeout: Duration,
) -> tokio::io::Result<TcpStream> {
    let mut join_set = race_addrs(addrs, timeout).await;
    while let Some(result) = join_set.join_next().await {
        if let Ok(Ok((_, stream))) = result {
            return Ok(stream);
        }
    }
    Err(tokio::io::Error::new(
        tokio::io::ErrorKind::NotConnected,
        "NotConnected",
    ))
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use super::*;

    #[tokio::test]
    async fn test_sort() {
        let addrs = vec![
            SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 80),
            SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 8000),
        ];
        let out = race_addrs(addrs.clone(), Duration::from_secs(10)).await;
        println!("{out:?}");
        let str = tcp_connect_fastest(addrs, Duration::from_secs(10)).await;
        println!("{str:?}");
    }
}
