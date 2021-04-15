### To Run the web frontend
To run, install [trunk](https://crates.io/crates/trunk).
**Make sure to run `cargo install wasm-bindgen-cli` in the instructions.**

To run the code, switch to the `frontend` folder and just use
```
trunk serve
```
Or you can do it in one-shot with
```
(cd frontend && trunk serve)
```
**Note** The current rust versions have an error with stdweb. A known working version of rust can be installed with:
```
rustup override set 1.45.0
```
This should be used in the `frontend` directory since override sets the current directory to use the rust version.
There may be an issue with wasm-unknown-unknown, to fix run
```
rustup target add wasm32-unknown-unknown
```

### To Run MongoDB backend
1. Make sure MongoDB is installed on your machine, detail of installation can be found [here](https://docs.mongodb.com/manual/installation/)
2. Then ensure MongoDB is running on the default port (27017).
3. The backend uses [rocket](https://github.com/SergioBenitez/Rocket/tree/v0.4), which requires Rust nightly to compile, if you're not using Rust nightly, you can perform the switch by using 
```
rustup override set nightly
```
**Make sure the above command is performed at the root directory of this project**

4. Run the backend by switching to the `backend` folder and use
```
cargo run
```
Or you can also do it in one-shot with
```
(cd backend && cargo run)
```
A rocket will be launched at `127.0.0.1:8000`, opening that URL is not necessary (there is nothing at that URL), the URL is used to serve HTTP `POST` and `GET` requests
