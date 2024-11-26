use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    http::{header, HeaderMap},
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on {}", path, addr);

    let state = HttpServeState { path: path.clone() };

    let router = Router::new()
        .nest_service("/tower", ServeDir::new(path))
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, HeaderMap, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            HeaderMap::new(),
            format!("File {:?} not found", p.display()),
        )
    } else {
        // TODO: test p is a directory
        // if it is a directory, list all files/subdirectories
        // as <li><a href="/path/to/file">file name</a></li>
        // <html><body><ul>...</ul></body></html>
        if p.is_dir() {
            let mut item = String::new();
            for entry in p.read_dir().expect("read_dir call failed") {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let path_str = path.display().to_string().replace("\\", "/");
                    let file_name = format!(
                        "<li><a href=\"{}\">{}</a></li>",
                        path_str,
                        path.file_name().unwrap().to_str().unwrap()
                    );
                    item.push_str(&file_name);
                }
            }
            let mut headers = HeaderMap::new();
            headers.insert(header::CONTENT_TYPE, "text/html".parse().unwrap());

            return (
                StatusCode::OK,
                headers,
                format!("<!DOCTYPE html><html><body><ul>{}</ul></body></html>", item),
            );
        }

        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, HeaderMap::new(), content)
            }
            Err(e) => {
                warn!("Error reading file {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    HeaderMap::new(),
                    e.to_string(),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let (status, _, content) = file_handler(State(state), Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        assert!(content.trim().starts_with("[package]"));
    }
}
