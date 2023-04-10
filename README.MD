### Super simple client server calculator

#### How to run

1. build project with `cargo build --release`
2. create `config.json` file in the following format:
```json
{"port": 8080, "name": "calculator"}
```
3. start server and client (need to run production binaries)
 specifying path to config file, e.g. this will work from repository root
```shell
target/release/client --config config_example.json
target/release/server --config config_example.json
```
4. enter equation in the client cli

#### Notes
1. did not have time to do the tests, but it has been tested manually to ensure it works past config file load
2. u16 may have underflow or overflow. instead of sending an error, I convert the input to larger `u` or `f` type to make sure result fits size wize
