# Vaghar

Vaghar is a tool to find the passphrase for an Ethereum wallet by brute-forcing words combination. The user can specify the words and the pattern in which they should be tried using a configuration file in TOML format. The default configuration file is default.conf.

To run Vaghar with a custom configuration, use the --config command followed by the path to the configuration file. For example, ./vaghar --config my_config.conf.

Vaghar supports multi-core processing to parallelize the finding process, making it faster and more efficient.

## Build

To build Vaghar, you will need the Rust programming language and Cargo, its package manager. You can install both by following the instructions on the Rust website.

Once you have Rust and Cargo installed, navigate to the directory where you have cloned the Vaghar repository and run the following command:

```
cargo build --release
```

This will build a release version of Vaghar, optimized for performance. The binary will be located at ./target/release/vaghar.

## Usage

To run Vaghar, simply navigate to the directory where you have built the binary and run the following command:

```
./vaghar
```

This will use the default configuration file default.conf. If you want to use a custom configuration file, use the --config flag as mentioned above.

For example, to use the my_config.conf file, run the following command:

```
./vaghar --config my_config.conf
```

You can use `default.conf` as a starting point for creating your own configuration file.

Vaghar will then start the brute-forcing process, using the words and pattern specified in the configuration file. It will print the progress and the passphrase once it is found.

Please note that the brute-forcing process can take a long time, depending on the complexity of the passphrase and the number of cores available. Be patient and do not interrupt the process unless necessary.

[] Robin