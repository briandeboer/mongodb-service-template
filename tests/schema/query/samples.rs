#[cfg(test)]
mod test {
    use crate::utils;

    use actix_web::{test, App};
    use insta::assert_snapshot;
    use {{crate_name}}::routes::app_routes;

    use crate::schema::fragments;

    #[actix_rt::test]
    async fn all_samples() {
        std::env::set_var("BASE_PATH", "test_path");
        std::env::set_var("DISABLE_AUTH", "1");

        let mut app = test::init_service(
            App::new()
                .configure(utils::load_filled_database)
                .configure(app_routes),
        )
        .await;

        let query = utils::GqlQuery {
            operation_name: "allSamples",
            query: &format!(
                r#"
                query allSamples {{
                    allSamples(limit: 5, after: null, before: null) {{
                        totalCount
                        {page_info_fragment}
                        items {{
                            {sample_fragment}  
                        }}
                    }}
                }}"#,
                page_info_fragment = fragments::page_info(),
                sample_fragment = fragments::sample()
            ),
        };

        let req = test::TestRequest::post()
            .set_json(&query)
            .uri("/test_path/graphql")
            .to_request();

        let resp = test::read_response(&mut app, req).await;
        assert_snapshot!("all_samples", format!("{:?}", resp));
    }

    #[actix_rt::test]
    async fn expired_samples() {
        std::env::set_var("BASE_PATH", "test_path");
        std::env::set_var("DISABLE_AUTH", "1");

        let mut app = test::init_service(
            App::new()
                .configure(utils::load_filled_database)
                .configure(app_routes),
        )
        .await;

        let query = utils::GqlQuery {
            operation_name: "expiredSamples",
            query: &format!(
                r#"
                query expiredSamples {{
                    samplesByStatus(limit: 5, status: EXPIRED, after: null, before: null) {{
                        totalCount
                        {page_info_fragment}
                        items {{
                            {sample_fragment}  
                        }}
                    }}
                }}"#,
                page_info_fragment = fragments::page_info(),
                sample_fragment = fragments::sample()
            ),
        };

        let req = test::TestRequest::post()
            .set_json(&query)
            .uri("/test_path/graphql")
            .to_request();

        let resp = test::read_response(&mut app, req).await;
        assert_snapshot!("expired_samples", format!("{:?}", resp));
    }

    #[actix_rt::test]
    async fn pending_samples() {
        std::env::set_var("BASE_PATH", "test_path");
        std::env::set_var("DISABLE_AUTH", "1");

        let mut app = test::init_service(
            App::new()
                .configure(utils::load_filled_database)
                .configure(app_routes),
        )
        .await;

        let query = utils::GqlQuery {
            operation_name: "pendingSamples",
            query: &format!(
                r#"
                query pendingSamples {{
                    samplesByStatus(limit: 5, status: PENDING, after: null, before: null) {{
                        totalCount
                        {page_info_fragment}
                        items {{
                            {sample_fragment}  
                        }}
                    }}
                }}"#,
                page_info_fragment = fragments::page_info(),
                sample_fragment = fragments::sample()
            ),
        };

        let req = test::TestRequest::post()
            .set_json(&query)
            .uri("/test_path/graphql")
            .to_request();

        let resp = test::read_response(&mut app, req).await;
        assert_snapshot!("pending_samples", format!("{:?}", resp));
    }

    #[actix_rt::test]
    async fn all_samples_with_status() {
        std::env::set_var("BASE_PATH", "test_path");
        std::env::set_var("DISABLE_AUTH", "1");

        let mut app = test::init_service(
            App::new()
                .configure(utils::load_filled_database)
                .configure(app_routes),
        )
        .await;

        let query = utils::GqlQuery {
            operation_name: "allSamples",
            query: &format!(
                r#"
                query allSamples {{
                    samplesByStatus(limit: 5, status: ALL, after: null, before: null) {{
                        totalCount
                        {page_info_fragment}
                        items {{
                            {sample_fragment}  
                        }}
                    }}
                }}"#,
                page_info_fragment = fragments::page_info(),
                sample_fragment = fragments::sample()
            ),
        };

        let req = test::TestRequest::post()
            .set_json(&query)
            .uri("/test_path/graphql")
            .to_request();

        let resp = test::read_response(&mut app, req).await;
        assert_snapshot!("all_samples_with_status", format!("{:?}", resp));
    }

    #[actix_rt::test]
    async fn sample_by_id_existing() {
        std::env::set_var("BASE_PATH", "test_path");
        std::env::set_var("DISABLE_AUTH", "1");

        let mut app = test::init_service(
            App::new()
                .configure(utils::load_filled_database)
                .configure(app_routes),
        )
        .await;

        let query = utils::GqlQuery {
            operation_name: "sampleById",
            query: &format!(
                r#"
                query sampleById {{
                    sampleById(id: "$oid:5f192d9900e0306000d188e1") {{
                        {sample_fragment}
                    }}
                }}"#,
                sample_fragment = fragments::sample()
            ),
        };

        let req = test::TestRequest::post()
            .set_json(&query)
            .uri("/test_path/graphql")
            .to_request();

        let resp = test::read_response(&mut app, req).await;
        assert_snapshot!("sample_by_id_existing", format!("{:?}", resp));
    }

    #[actix_rt::test]
    async fn sample_by_non_existent() {
        std::env::set_var("BASE_PATH", "test_path");
        std::env::set_var("DISABLE_AUTH", "1");

        let mut app = test::init_service(
            App::new()
                .configure(utils::load_filled_database)
                .configure(app_routes),
        )
        .await;

        let query = utils::GqlQuery {
            operation_name: "sampleById",
            query: &format!(
                r#"
                query sampleById {{
                    sampleById(id: "$oid:NOOBJECT") {{
                        {sample_fragment}
                    }}
                }}"#,
                sample_fragment = fragments::sample()
            ),
        };

        let req = test::TestRequest::post()
            .set_json(&query)
            .uri("/test_path/graphql")
            .to_request();

        let resp = test::read_response(&mut app, req).await;
        assert_snapshot!("sample_by_non_existent", format!("{:?}", resp));
    }

    #[actix_rt::test]
    async fn sample_by_names() {
        std::env::set_var("BASE_PATH", "test_path");
        std::env::set_var("DISABLE_AUTH", "1");

        let mut app = test::init_service(
            App::new()
                .configure(utils::load_filled_database)
                .configure(app_routes),
        )
        .await;

        let query = utils::GqlQuery {
            operation_name: "sampleByNames",
            query: &format!(
                r#"
                query sampleByNames {{
                    sampleByNames(names: ["Sample 1", "Sample 3"]) {{
                        totalCount
                        {page_info_fragment}
                        items {{
                            {sample_fragment}  
                        }}
                    }}
                }}"#,
                page_info_fragment = fragments::page_info(),
                sample_fragment = fragments::sample()
            ),
        };

        let req = test::TestRequest::post()
            .set_json(&query)
            .uri("/test_path/graphql")
            .to_request();

        let resp = test::read_response(&mut app, req).await;
        assert_snapshot!("sample_by_names", format!("{:?}", resp));
    }

    #[actix_rt::test]
    async fn sample_by_names_missing() {
        std::env::set_var("BASE_PATH", "test_path");
        std::env::set_var("DISABLE_AUTH", "1");

        let mut app = test::init_service(
            App::new()
                .configure(utils::load_filled_database)
                .configure(app_routes),
        )
        .await;

        let query = utils::GqlQuery {
            operation_name: "sampleByNames",
            query: &format!(
                r#"
                query sampleByNames {{
                    sampleByNames(names: ["Sample 1", "Sample 3", "Nope"]) {{
                        totalCount
                        {page_info_fragment}
                        items {{
                            {sample_fragment}  
                        }}
                    }}
                }}"#,
                page_info_fragment = fragments::page_info(),
                sample_fragment = fragments::sample()
            ),
        };

        let req = test::TestRequest::post()
            .set_json(&query)
            .uri("/test_path/graphql")
            .to_request();

        let resp = test::read_response(&mut app, req).await;
        assert_snapshot!("sample_by_names_missing", format!("{:?}", resp));
    }
}
