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

### Building the project

1.  Install the dependencies:

    ```
    make install-deps
    ```

2.  Execute `make` to compile the code, build the documentation and run tests.

### Running the project

You can run the built binary with the following command:

```
make run ENV=<env_file>
```

For example, to run the project locally, execute the following command:

```
make run ENV=env.local
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
