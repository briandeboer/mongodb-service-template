# Sample Service

## Getting Started
- Install [Rust](https://www.rust-lang.org/tools/install)
- Run `cargo run` to build and run service


### Install Docker and Docker Compose (Optional)
- [Docker](https://docs.docker.com/engine/install/)
- [docker-compose](https://docs.docker.com/compose/install/)


### Start Mongo
You can skip this step if you prefer to run a local Mongo instance.

Using docker-compose
```
docker-compose up -d mongo
```
In the future you can run `docker start mongo` to relaunch the service.


## VSCode

### Plugins
- Better TOML
- Native Debug
- Rust
- rust-analyzer

### Debugging
Run debug (F5) to create a **`launch.json`** file, make sure the value of **target** is pointed to your built executable, usually `./target/rust-deps/debug/{{project-name}}` unless you changed `$CARGO_TARGET_DIR`


## Graphiql

- http://localhost:8084/{{project-name}}/graphiql

## Docker
You can run the service with docker-compose. It currently doesn't take into consideration the login-service, but that is something to look into the best way to accomplish.

### Build:
```
docker build -t {{project-name}}:latest .
```

### Run
```
docker-compose up -d
```

## Tests
In order to run tests against graphql you will need to have a local runnning mongo server on port 27017. You can use docker-compose or whatever method you want to have that. It will automatically create a new database just for tests named `{{project-name}}-test`. In order for your test to leverage the database you should configure your test app with the `load-filled-database`. That will read all mock data from the `tests/mock` folder and insert them into the database. Here's an example...

```rust
use crate::utils;

...

let mut app = test::init_service(
    App::new()
        .configure(utils::load_filled_database)
        .configure(app_routes),
).await;

```

For more information, look at the `schema/query/users.rs` tests. For most tests, using test snapshots will work well. For more information on snapshot testing see [here](https://jestjs.io/docs/en/snapshot-testing#best-practices). The tests rely on the `insta` [crate](https://github.com/mitsuhiko/insta). First install insta follow the instructions on their site:

```sh
cargo install cargo-insta
```


To run the tests first make sure that you have mongo running on local port 27084 or use docker-compose:

```sh
docker-compose up -d mongo
```

```sh
./test
```

If any of your tests fail or have new snapshots, you can review them with:

```sh
cargo insta review
```

### Time manipulation

For some snapshot tests you'll need to lock SystemTime to a fixed number to prevent things like `date_modified` or `date_created` updates to differ between snapshots. Because mongodb-base-service automatically updates the objects with those times, it has been updated to allow for mocking time. This already happens inside the `load_filled_database` function. But if you need to override it or fill the database with your own distinct mock data, you can set the time to a specific number:

```rust
// fix time to Jan 1, 2020 so that snapshots always have the same date_created, etc...
mock_time::set_mock_time(SystemTime::UNIX_EPOCH + Duration::from_millis(1577836800000)); 
```

Or if you need to increase the time to verify that the `date_modified` has changed:

```rust
// increase time by 10 seconds
mock_time::increase_mock_time(10000);
```

If you want to reset the time to normal SystemTime, you can use:

```rust
// revert to normal SystemTime
mock_time::clear_mock_time();
```

### Notes on multi-threading

It's important to understand that parallel tests which all write to a single database or manipulate `SystemTime` can cause issues and break tests. For that reason to ensure that the tests all run independently you should use:

```sh
cargo test --jobs=1 -- --test-threads=1
```

When you run them with concurrenncy disabled that does mean that insta will fail. In order to resolve that be sure that all tests include a snapshot name

```rust
assert_snapshot!("test_snapshot_name", format!("{:?}", resp));
```
