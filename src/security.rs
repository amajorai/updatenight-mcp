pub fn terminal_text(value: &str) -> String {
    value.chars().filter(|ch| !ch.is_control()).collect()
}

pub fn http_url(value: &str) -> anyhow::Result<reqwest::Url> {
    anyhow::ensure!(
        !value.chars().any(char::is_control),
        "URL contains control characters"
    );
    let url = reqwest::Url::parse(value)?;
    let host = url
        .host_str()
        .ok_or_else(|| anyhow::anyhow!("Expected a URL host"))?;
    let loopback = host.eq_ignore_ascii_case("localhost")
        || host
            .parse::<std::net::IpAddr>()
            .is_ok_and(|address| address.is_loopback());
    anyhow::ensure!(
        (url.scheme() == "https" || (url.scheme() == "http" && loopback))
            && url.username().is_empty()
            && url.password().is_none(),
        "Expected HTTPS or loopback HTTP without credentials"
    );
    Ok(url)
}

pub fn http_client() -> anyhow::Result<reqwest::Client> {
    Ok(reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?)
}

pub fn open_url(value: &str) -> anyhow::Result<()> {
    let url = http_url(value)?;
    open::that_detached(url.as_str())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn browser_urls_are_web_urls_and_terminal_controls_are_removed() {
        for url in [
            "https://example.com/a?x=%22%26",
            "http://localhost:3000/verify",
        ] {
            assert!(http_url(url).is_ok());
        }
        for url in [
            "file:///tmp/x",
            "http://example.com/verify",
            "https://user:pass@example.com",
            "https://example.com/\n",
            "javascript:alert(1)",
        ] {
            assert!(http_url(url).is_err());
        }
        assert_eq!(
            terminal_text("hello\x1b[31m\x07\r\u{009b}世界"),
            "hello[31m世界"
        );
    }

    #[tokio::test]
    async fn authenticated_client_does_not_follow_redirects() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let destination = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let destination_url = format!("http://{}/token", destination.local_addr().unwrap());
        let redirect = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let redirect_url = format!("http://{}/start", redirect.local_addr().unwrap());

        let redirect_response = tokio::spawn(async move {
            let (mut stream, _) = redirect.accept().await.unwrap();
            let mut request = vec![0; 4096];
            let _ = stream.read(&mut request).await.unwrap();
            stream
                .write_all(
                    format!(
                        "HTTP/1.1 302 Found\r\nLocation: {destination_url}\r\nContent-Length: 0\r\n\r\n"
                    )
                    .as_bytes(),
                )
                .await
                .unwrap();
        });
        let destination_request =
            tokio::time::timeout(std::time::Duration::from_millis(500), destination.accept());

        let response = http_client()
            .unwrap()
            .get(redirect_url)
            .bearer_auth("secret-token")
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), reqwest::StatusCode::FOUND);
        redirect_response.await.unwrap();
        assert!(destination_request.await.is_err());
    }
}
