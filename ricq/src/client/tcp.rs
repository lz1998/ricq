use std::net::SocketAddr;
use std::time::Duration;

use futures::StreamExt;
use tokio::net::TcpStream;

pub async fn tcp_connect_timeout<Addr>(
    addr: Addr,
    timeout: Duration,
) -> tokio::io::Result<TcpStream>
where
    SocketAddr: From<Addr>,
{
    tokio::time::timeout(
        timeout,
        tokio::net::TcpStream::connect(SocketAddr::from(addr)),
    )
    .await
    .map_err(tokio::io::Error::from)
    .flatten()
}

pub async fn tcp_ping<Addr>(addr: Addr, timeout: Duration) -> Duration
where
    SocketAddr: From<Addr>,
{
    let start = std::time::Instant::now();
    match tcp_connect_timeout(addr, timeout).await {
        Ok(_) => start.elapsed(),
        Err(_) => timeout,
    }
}

pub async fn sort_addrs<Addr>(addrs: Vec<Addr>, timeout: Duration) -> Vec<Addr>
where
    SocketAddr: From<Addr>,
    Addr: Clone,
{
    let len = addrs.len();
    let mut result = futures::stream::iter(addrs)
        .map(|addr| async move { (addr.clone(), tcp_ping(addr, timeout).await) })
        .buffer_unordered(len)
        .collect::<Vec<_>>()
        .await;
    result.sort_unstable_by_key(|(_, duration)| *duration);
    result.into_iter().map(|(addr, _)| addr).collect()
}

pub async fn tcp_connect_fastest<Addr>(
    addrs: Vec<Addr>,
    timeout: Duration,
) -> tokio::io::Result<TcpStream>
where
    SocketAddr: From<Addr>,
{
    let len = addrs.len();
    let mut output = futures::stream::iter(addrs)
        .map(|addr| tcp_connect_timeout(addr, timeout))
        .buffer_unordered(len);
    while let Some(result) = output.next().await {
        if let Ok(stream) = result {
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
        let out = sort_addrs(addrs.clone(), Duration::from_secs(10)).await;
        println!("{:?}", out);
        let str = tcp_connect_fastest(addrs, Duration::from_secs(10)).await;
        println!("{:?}", str);
    }
}
