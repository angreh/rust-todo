//integration test
use hyper::{Client, StatusCode};

#[tokio::test]
async fn home() {
    let client = Client::new();
    let uri = "http://localhost:8080/".parse().unwrap();
    let resp = client.get(uri).await.unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(resp.into_body()).await.unwrap();
    assert_eq!(&body[..], b"rust api");
}
