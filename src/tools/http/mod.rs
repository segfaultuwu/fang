pub mod get;
pub mod post;
pub mod delete;
pub mod put;
pub mod spider;

pub async fn request(
	method: reqwest::Method,
	url: &str,
	params: Option<&[(&str, &str)]>,
	body: Option<&str>,
) -> anyhow::Result<()> {
	let client = reqwest::Client::new();
	let mut url = reqwest::Url::parse(url)?;

	if let Some(params) = params {
		let mut pairs = url.query_pairs_mut();
		for (key, value) in params {
			pairs.append_pair(key, value);
		}
	}

	let mut request = client.request(method, url);

	if let Some(body) = body {
		request = request.body(body.to_owned());
	}

	let response = request.send().await?;
	println!("Status: {}", response.status());
	let body = response.text().await?;
	println!("Body:\n{}", body);

	Ok(())
}