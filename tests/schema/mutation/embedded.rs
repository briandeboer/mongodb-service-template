#[cfg(test)]
mod test {
    use crate::utils;

    use actix_web::{test, App};
    use insta::assert_snapshot;
    use mongodb_base_service::mock_time;
    use sample_project::routes::app_routes;

    use crate::schema::fragments;

    #[actix_rt::test]
    async fn add_embedded_object() {
        std::env::set_var("BASE_PATH", "test_path");
        std::env::set_var("DISABLE_AUTH", "1");

        let mut app = test::init_service(
            App::new()
                .configure(utils::load_filled_database)
                .configure(app_routes),
        )
        .await;

        let query = utils::GqlQuery {
            operation_name: "updateSample",
            query: &format!(
                r#"
                mutation updateSample {{
                    updateSample(
                        id: "$oid:5f192d9900e0306000d188e1"
                        updateSample: {{
                            name: "New Name"
                            description: "New Description"
                        }}
                    ) {{
                        {sample_fragment}  
                    }}
                }}"#,
                sample_fragment = fragments::sample()
            ),
        };

        // increase time so date modified updates
        mock_time::increase_mock_time(10000);

        let req = test::TestRequest::post()
            .set_json(&query)
            .uri("/test_path/graphql")
            .to_request();

        let resp = test::read_response(&mut app, req).await;
        assert_snapshot!("update_existing_sample", format!("{:?}", resp));
    }

    #[actix_rt::test]
    async fn update_non_existent_sample() {
        std::env::set_var("BASE_PATH", "test_path");
        std::env::set_var("DISABLE_AUTH", "1");

        let mut app = test::init_service(
            App::new()
                .configure(utils::load_filled_database)
                .configure(app_routes),
        )
        .await;

        let query = utils::GqlQuery {
            operation_name: "updateSample",
            query: &format!(
                r#"
                mutation updateSample {{
                    updateSample(
                        id: "$oid:NO_OBJECT"
                        updateSample: {{
                            name: "New Name"
                            description: "New Description"
                        }}
                    ) {{
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
        assert_snapshot!("update_non_existent_sample", format!("{:?}", resp));
    }

    #[actix_rt::test]
    async fn bad_update() {
        std::env::set_var("BASE_PATH", "test_path");
        std::env::set_var("DISABLE_AUTH", "1");

        let mut app = test::init_service(
            App::new()
                .configure(utils::load_filled_database)
                .configure(app_routes),
        )
        .await;

        let query = utils::GqlQuery {
            operation_name: "updateSample",
            query: &format!(
                r#"
                mutation updateSample {{
                    updateSample(
                        id: "$oid:5f192d9900e0306000d188e1"
                        updateSample: {{
                            thisFieldDoesNotExist: "something"
                            name: "New Name"
                            description: "New Description"
                        }}
                    ) {{
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
        assert_snapshot!("bad_update", format!("{:?}", resp));
    }
}
