{% assign lbrace = "{" %}
{% assign rbrace = "}" %}

#[cfg(test)]
mod test {
    use crate::utils;

    use actix_web::{test, App};
    use insta::assert_snapshot;
    use mongodb_base_service::mock_time;
    use {% lbrace %}crate_name{% rbrace %}::routes::app_routes;

    use crate::schema::fragments;

    #[actix_rt::test]
    async fn create_sample() {
        std::env::set_var("BASE_PATH", "test_path");
        std::env::set_var("DISABLE_AUTH", "1");

        let mut app = test::init_service(
            App::new()
                .configure(utils::load_filled_database)
                .configure(app_routes),
        )
        .await;

        let query = utils::GqlQuery {
            operation_name: "createSample",
            query: &format!(
                r#"
                mutation createSample {% lbrace %}
                    createSample(
                        newSample: {% lbrace %}
                            id: "$oid:5f5be6b800ca625d0066cf3e"
                            name: "Brand New Name"
                            description: "Brand New Description"
                            availableDate: 1
                            expirationDate: 2147483647
                        {% rbrace %}
                    ) {% lbrace %}
                        {sample_fragment}  
                    {% rbrace %}
                {% rbrace %}"#,
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
        assert_snapshot!("create_sample", format!("{:?}", resp));

        // go check if it's in the all list
        let query = utils::GqlQuery {
            operation_name: "allSamples",
            query: &format!(
                r#"
                query allSamples {% lbrace %}
                    allSamples(limit: 5, after: null, before: null) {% lbrace %}
                        totalCount
                        {page_info_fragment}
                        items {% lbrace %}
                            {sample_fragment}
                        {% rbrace %}
                    {% rbrace %}
                {% rbrace %}"#,
                page_info_fragment = fragments::page_info(),
                sample_fragment = fragments::sample()
            ),
        };
        let req = test::TestRequest::post()
            .set_json(&query)
            .uri("/test_path/graphql")
            .to_request();

        let resp = test::read_response(&mut app, req).await;
        assert_snapshot!("create_sample_all_samples", format!("{:?}", resp));
    }

    #[actix_rt::test]
    async fn create_existing_sample() {
        std::env::set_var("BASE_PATH", "test_path");
        std::env::set_var("DISABLE_AUTH", "1");

        let mut app = test::init_service(
            App::new()
                .configure(utils::load_filled_database)
                .configure(app_routes),
        )
        .await;

        let query = utils::GqlQuery {
            operation_name: "createSample",
            query: &format!(
                r#"
                mutation createSample {% lbrace %}
                    createSample(
                        newSample: {% lbrace %}
                            id: "$oid:5f192d9900e0306000d188e1"
                            name: "Brand New Name"
                            description: "Brand New Description"
                            availableDate: 1
                            expirationDate: 2147483647
                        {% rbrace %}
                    ) {% lbrace %}
                        {sample_fragment}  
                    {% rbrace %}
                {% rbrace %}"#,
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
        assert_snapshot!("create_existing_sample", format!("{:?}", resp));
    }

    #[actix_rt::test]
    async fn update_existing_sample() {
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
                mutation updateSample {% lbrace %}
                    updateSample(
                        id: "$oid:5f192d9900e0306000d188e1"
                        updateSample: {% lbrace %}
                            name: "New Name"
                            description: "New Description"
                        {% rbrace %}
                    ) {% lbrace %}
                        {sample_fragment}  
                    {% rbrace %}
                {% rbrace %}"#,
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
                mutation updateSample {% lbrace %}
                    updateSample(
                        id: "$oid:NO_OBJECT"
                        updateSample: {% lbrace %}
                            name: "New Name"
                            description: "New Description"
                        {% rbrace %}
                    ) {% lbrace %}
                        {sample_fragment}  
                    {% rbrace %}
                {% rbrace %}"#,
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
                mutation updateSample {% lbrace %}
                    updateSample(
                        id: "$oid:5f192d9900e0306000d188e1"
                        updateSample: {% lbrace %}
                            thisFieldDoesNotExist: "something"
                            name: "New Name"
                            description: "New Description"
                        {% rbrace %}
                    ) {% lbrace %}
                        {sample_fragment}  
                    {% rbrace %}
                {% rbrace %}"#,
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

    #[actix_rt::test]
    async fn delete_existing_sample() {
        std::env::set_var("BASE_PATH", "test_path");
        std::env::set_var("DISABLE_AUTH", "1");

        let mut app = test::init_service(
            App::new()
                .configure(utils::load_filled_database)
                .configure(app_routes),
        )
        .await;

        let query = utils::GqlQuery {
            operation_name: "deleteSample",
            query: &format!(
                r#"
                mutation deleteSample {% lbrace %}
                    deleteSample(
                        id: "$oid:5f192d9900e0306000d188e1"
                    ) {% lbrace %}
                        id
                        success
                    {% rbrace %}
                {% rbrace %}"#,
            ),
        };

        let req = test::TestRequest::post()
            .set_json(&query)
            .uri("/test_path/graphql")
            .to_request();

        let resp = test::read_response(&mut app, req).await;
        assert_snapshot!("delete_existing_sample", format!("{:?}", resp));

        // go check if it's in the all list
        let query = utils::GqlQuery {
            operation_name: "allSamples",
            query: &format!(
                r#"
                query allSamples {% lbrace %}
                    allSamples(limit: 5, after: null, before: null) {% lbrace %}
                        totalCount
                        {page_info_fragment}
                        items {% lbrace %}
                            {sample_fragment}
                        {% rbrace %}
                    {% rbrace %}
                {% rbrace %}"#,
                page_info_fragment = fragments::page_info(),
                sample_fragment = fragments::sample()
            ),
        };
        let req = test::TestRequest::post()
            .set_json(&query)
            .uri("/test_path/graphql")
            .to_request();

        let resp = test::read_response(&mut app, req).await;
        assert_snapshot!("delete_sample_all_samples", format!("{:?}", resp));
    }
}
