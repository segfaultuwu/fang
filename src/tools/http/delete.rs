pub async fn url(url: &str, params: Option<&[(&str, &str)]>) -> anyhow::Result<()> {
	crate::tools::http::request(reqwest::Method::DELETE, url, params, None).await
}
