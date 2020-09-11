#[cfg(test)]
mod health_tests {
    use actix_service::Service;
    use actix_web::{http::StatusCode, test, App};
    use bytes::Bytes;
    use {{crate_name}}::routes::app_routes;

    #[actix_rt::test]
    async fn test_pong() {
        std::env::set_var("BASE_PATH", "test_path");
        let mut app = test::init_service(App::new().configure(app_routes)).await;

        let req = test::TestRequest::get().uri("/test_path/ping").to_request();
        let resp = test::read_response(&mut app, req).await;

        assert_eq!(resp, Bytes::from_static(b"pong"));
    }

    #[actix_rt::test]
    async fn test_health() {
        std::env::set_var("BASE_PATH", "test_path");
        let mut app = test::init_service(App::new().configure(app_routes)).await;

        let req = test::TestRequest::get()
            .uri("/test_path/health")
            .to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let result = test::read_body(resp).await;
        assert_eq!(result, Bytes::from_static(b"OK"));
    }

    #[actix_rt::test]
    async fn test_readiness_ok() {
        std::env::set_var("BASE_PATH", "test_path");
        let mut app = test::init_service(App::new().configure(app_routes)).await;

        let req = test::TestRequest::with_uri("/test_path/~/ready").to_request();

        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::ACCEPTED);
    }

    #[actix_rt::test]
    async fn not_found_route() {
        std::env::set_var("BASE_PATH", "test_path");
        let mut app = test::init_service(App::new().configure(app_routes)).await;

        let req = test::TestRequest::get()
            .uri("/test_path/crazy-route")
            .to_request();

        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}
