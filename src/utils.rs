use actix_web::HttpRequest;

pub fn get_ip(req: HttpRequest) -> String {
    req.headers()
        .get("X-Forwarded-For")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            req.connection_info()
                .realip_remote_addr()
                .unwrap_or("unknown")
                .to_string()
        })
}
