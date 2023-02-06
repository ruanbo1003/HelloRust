
## control test run
### run subset of tests
* run single test, specify the test function name
```shell
cargo test not_equal_test
```

* run filtered tests, by specify the filter-word
```shell
cargo test "equal"
```

* ignore test unless specifically requested
    * run the ignored tests only
    ```shell
    cargo test -- --ignored
    ```
    * run all tests include ignored tests
    ```shell
    cargo test -- --include-ignored
    ```
  
## integration test for library crate, not for binary crate.
  the integration tests files are located in tests/ folder, and the 
  tests/ folder is next to the src/ folder, not in the src/ folder.
  
* only run the integration test
```shell
cargo test --test integration_test
```

