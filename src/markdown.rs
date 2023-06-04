use serde_json::json;

const GITHUB_MARKDOWN_URL: &str = "https://api.github.com/markdown";

pub async fn markdown_to_html(md: &str) -> String {
    let payload = json!({ "text": md });
    let request = gloo_net::http::Request::post(GITHUB_MARKDOWN_URL)
        .json(&payload)
        .unwrap();
    let response = request.send().await.unwrap();
    let html = response.text().await.unwrap();
    crate::log(&format!("HTML: {html}"));
    html
}
