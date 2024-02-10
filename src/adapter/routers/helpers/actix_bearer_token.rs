use actix_web::HttpRequest;

pub fn extract_bearer_token(req: &HttpRequest) -> Option<&str> {
  req.headers().get("Authorization").and_then(|header| {
    let value = header.to_str().ok()?;
    let parts: Vec<&str> = value.split_whitespace().collect();
    if parts.len() == 2 && parts[0] == "Bearer" {
      Some(parts[1])
    } else {
      None
    }
  })
}
