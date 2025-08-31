use axum::body::Body;
use axum::extract::Path;
use axum::http::{HeaderValue, Response, StatusCode, header};
use axum::response::IntoResponse;
use tokio_util::codec::{BytesCodec, FramedRead};
use web_assets::files::StaticFile;

pub async fn static_path(Path(path): Path<String>) -> impl IntoResponse {
    let path = format!("/static/{}", path);
    let data = StaticFile::get(&path);

    if let Some(data) = data {
        let file = match tokio::fs::File::open(data.file_name).await {
            Ok(file) => file,
            Err(_) => {
                return Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .unwrap();
            }
        };

        // convert the `AsyncRead` into a `Stream`
        let stream = FramedRead::new(file, BytesCodec::new());
        return Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(data.mime.as_ref()).unwrap(),
            )
            .body(Body::from_stream(stream))
            .unwrap();
    }

    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap()
}
