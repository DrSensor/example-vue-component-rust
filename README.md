# example-vue-component-rust

[blog post](https://busy.org/@drsensor/mix-rust-code-webassembly-with-vue-component-basic)

#### Setup
install and setup rust toolchain first
```bash
rustup default nightly
rustup target add wasm32-unknown-unknown
cargo install wasm-gc
```
then
```bash
git clone git@github.com:DrSensor/example-vue-component-rust.git
yarn install
```

#### Run
```bash
yarn serve --open
```

#### Build
```bash
yarn build
```
the result are in `./dist` directory

## License
MIT