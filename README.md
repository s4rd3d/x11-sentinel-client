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

1.  Define the following variables:

    ```
    # local, staging, test or live
    $ export RUNTIME_ENV=...
    ```

2.  Install the dependencies:

    ```
    make install-deps
    ```

3.  Build the package:

    ```
    make compile
    ```

### Running the project

You can run the built binary with the following command:

```
make run
```

## License

The project is licensed under the
[MIT license](http://opensource.org/licenses/MIT) with an addition regarding the
origin of the project. For addition information see the [LICENSE](LICENSE) file.
