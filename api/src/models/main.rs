use axum::response::{Html, IntoResponse};

pub async fn handler_main() -> impl IntoResponse {
    println!("HANDLER: main");

    Html(format!("rust api"))
}

// Unit test
#[cfg(test)]
mod tests {
    #[test]
    fn handler_main() {
        assert_eq!(4, 2 + 2);
    }
}
