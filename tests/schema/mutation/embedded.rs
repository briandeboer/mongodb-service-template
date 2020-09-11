#[cfg(test)]
mod test {
    use crate::utils;

    use actix_web::{test, App};
    use insta::assert_snapshot;
    use mongodb_base_service::mock_time;
    use {{crate_name}}::routes::app_routes;

    use crate::schema::fragments;

    #[actix_rt::test]
    async fn add_values_to_sample() {
        std::env::set_var("BASE_PATH", "test_path");
        std::env::set_var("DISABLE_AUTH", "1");

        let mut app = test::init_service(
            App::new()
                .configure(utils::load_filled_database)
                .configure(app_routes),
        )
        .await;

        let query = utils::GqlQuery {
            operation_name: "addValuesToSample",
            query: &format!(
                r#"
                mutation addValuesToSample {{
                    addValuesToSample(
                        sampleId: "$oid:5f192d9900e0306000d188e1"
                        newValues: [{{
                            id: "12345"
                            embeddedType: ONE
                            value: 0.1
                        }}, {{
                            id: "12346"
                            embeddedType: ANOTHER
                        }}]
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
        assert_snapshot!("add_values_to_sample", format!("{:?}", resp));
    }

    #[actix_rt::test]
    async fn add_values_to_missing_sample() {
        std::env::set_var("BASE_PATH", "test_path");
        std::env::set_var("DISABLE_AUTH", "1");

        let mut app = test::init_service(
            App::new()
                .configure(utils::load_filled_database)
                .configure(app_routes),
        )
        .await;

        let query = utils::GqlQuery {
            operation_name: "addValuesToSample",
            query: &format!(
                r#"
                mutation addValuesToSample {{
                    addValuesToSample(
                        sampleId: "$oid:NOOBJECT"
                        newValues: [{{
                            id: "12345"
                            embeddedType: ONE
                            value: 0.1
                        }}, {{
                            id: "12346"
                            embeddedType: ANOTHER
                        }}]
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
        assert_snapshot!("add_values_to_missing_sample", format!("{:?}", resp));
    }

    #[actix_rt::test]
    async fn update_value_for_sample_existing() {
        std::env::set_var("BASE_PATH", "test_path");
        std::env::set_var("DISABLE_AUTH", "1");

        let mut app = test::init_service(
            App::new()
                .configure(utils::load_filled_database)
                .configure(app_routes),
        )
        .await;

        let query = utils::GqlQuery {
            operation_name: "updateValueForSample",
            query: &format!(
                r#"
                mutation updateValueForSample {{
                    updateValueForSample(
                        sampleId: "$oid:5f192d9900e0306000d188e1"
                        embeddedId: "44514a55-2abd-4388-8f77-b96b0b25fc30"
                        updateValue: {{
                            embeddedType: ONE
                            value: 0.123
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
        assert_snapshot!("update_value_for_sample", format!("{:?}", resp));
    }

    #[actix_rt::test]
    async fn update_value_for_sample_non_existing() {
        std::env::set_var("BASE_PATH", "test_path");
        std::env::set_var("DISABLE_AUTH", "1");

        let mut app = test::init_service(
            App::new()
                .configure(utils::load_filled_database)
                .configure(app_routes),
        )
        .await;

        let query = utils::GqlQuery {
            operation_name: "updateValueForSample",
            query: &format!(
                r#"
                mutation updateValueForSample {{
                    updateValueForSample(
                        sampleId: "$oid:5f192d9900e0306000d188e1"
                        embeddedId: "NO_ID"
                        updateValue: {{
                            embeddedType: ONE
                            value: 0.123
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
        assert_snapshot!(
            "update_value_for_sample_non_existing",
            format!("{:?}", resp)
        );
    }

    #[actix_rt::test]
    async fn remove_value_from_sample_existing() {
        std::env::set_var("BASE_PATH", "test_path");
        std::env::set_var("DISABLE_AUTH", "1");

        let mut app = test::init_service(
            App::new()
                .configure(utils::load_filled_database)
                .configure(app_routes),
        )
        .await;

        let query = utils::GqlQuery {
            operation_name: "removeValueFromSample",
            query: &format!(
                r#"
                mutation removeValueFromSample {{
                    removeValueFromSample(
                        sampleId: "$oid:5f192d9900e0306000d188e1"
                        embeddedId: "44514a55-2abd-4388-8f77-b96b0b25fc30"
                    ) {{
                        id
                        success
                    }}
                }}"#,
            ),
        };

        // increase time so date modified updates
        mock_time::increase_mock_time(10000);

        let req = test::TestRequest::post()
            .set_json(&query)
            .uri("/test_path/graphql")
            .to_request();

        let resp = test::read_response(&mut app, req).await;
        assert_snapshot!("remove_value_from_sample_existing", format!("{:?}", resp));
    }

    #[actix_rt::test]
    async fn remove_value_from_sample_non_existing() {
        std::env::set_var("BASE_PATH", "test_path");
        std::env::set_var("DISABLE_AUTH", "1");

        let mut app = test::init_service(
            App::new()
                .configure(utils::load_filled_database)
                .configure(app_routes),
        )
        .await;

        let query = utils::GqlQuery {
            operation_name: "removeValueFromSample",
            query: &format!(
                r#"
                mutation removeValueFromSample {{
                    removeValueFromSample(
                        sampleId: "$oid:5f192d9900e0306000d188e1"
                        embeddedId: "NO_OBJECT"
                    ) {{
                        id
                        success
                    }}
                }}"#,
            ),
        };

        // increase time so date modified updates
        mock_time::increase_mock_time(10000);

        let req = test::TestRequest::post()
            .set_json(&query)
            .uri("/test_path/graphql")
            .to_request();

        let resp = test::read_response(&mut app, req).await;
        assert_snapshot!(
            "remove_value_from_sample_existing_non_existing",
            format!("{:?}", resp)
        );
    }
}
