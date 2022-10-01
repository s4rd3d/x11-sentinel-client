# x11-sentinel-client

Mouse cursor data collector and screen locker application running under the X11
protocol, implemented in Rust.

## Building and running the project

Currently only compiling from source is supported.

### Dependencies

*   **cargo**
    Install version at least `1.60.0`. Further details on installing `cargo` can
    be found in the official [documentation](https://doc.rust-lang.org/cargo/getting-started/installation.html).

*   **pkg-config**
    To install `pkg-config` run the following command:

    ```
    apt install pkg-config
    ```

*   **suckless-tools**
    To install `suckless-tools` run the following command:

    ```
    apt install suckless-tools
    ```

### Building the project

1.  Install the dependencies:

    ```
    make install-deps
    ```

2.  Execute `make` to compile the code, build the documentation and run tests.

### Building the project with Docker

To build the project and generate a single executable binary issue the following
command:

```
docker build -o bin .
```

The application will be available under `bin/x11-sentinel-client`.

### Running the project

The application can be configured either via command line arguments or with
environment variables. The command line arguments take priority over the
environment variables. If neither are provided, then default values are used.

The application can be configured by defining the following environment
variables:

*   `APP_API_KEY_NAME`

    Name of the API key that is sent with every submission request.

*   `APP_API_KEY_VALUE`

    Value of the API key that is sent with every submission request.

*   `APP_BUFFER_SIZE_LIMIT`

    Upper limit for the event buffer's size. When the event buffer's size
    reaches this number it triggers a submission.

*   `APP_IDLE_TIMEOUT`

    If no new event is generated for this number of milliseconds, a submission
    gets triggered.

*   `APP_LOCK_ENABLED`

    Whether X session locking functionality is enabled.

*   `APP_LOCK_THRESHOLD`

    If the user's score is lower than this predefined constant and session
    locking is enabled, then the session locking utility is executed.

*   `APP_LOCK_UTILITY`

    X session lock utility program that is used to lock the session when needed.

*   `APP_METADATA_QUERY_INTERVAL`

    Query interval of the platform specific metadata in milliseconds.

*   `APP_STATUS_BASE_URL`

    Base URL of the status API endpoint.

*   `APP_STATUS_INTERVAL`

    Query interval of the client's status in seconds.

*   `APP_SUBMIT_URL`

    URL of the submit API endpoint.

*   `APP_USER_ID`

    User ID which identifies the current user.

You can run the built binary with the following command:

```
make run
```

For more information on configuring the application via command line arguments
execute the following command:

```
x11-sentinel-client --help
```

## Documentation

Generate the documentation and make it available in
`target/doc/x11_sentinel_client/index.html` with the following command:

```
make doc
```

## License

The project is licensed under the
[MIT license](http://opensource.org/licenses/MIT) with an addition regarding the
origin of the project. For addition information see the [LICENSE](LICENSE) file.
