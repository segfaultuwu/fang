pub async fn url(url: &str, params: Option<&[(&str, &str)]>) -> anyhow::Result<()> {
    crate::tools::http::request(reqwest::Method::GET, url, params, None).await
}