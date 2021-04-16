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
**Make sure the above command is performed at the `frontend` directory of this project, since override sets the current directory to use the rust version**

There may be an issue with wasm-unknown-unknown, to fix run
```
rustup target add wasm32-unknown-unknown
```

### To Run MongoDB backend
Backend is used to get and store the histories of games, to run:
1. Make sure MongoDB is installed on your machine, detail of installation can be found [here](https://docs.mongodb.com/manual/installation/)
2. Then ensure MongoDB is running on the default port (27017).
3. The backend uses [rocket](https://github.com/SergioBenitez/Rocket/tree/v0.4), which requires Rust nightly to compile, if you're not using Rust nightly, you can perform the switch by using 
```
rustup override set nightly
```
**Make sure the above command is performed at the `backend` directory of this project, since override sets the current directory to use the rust version**

4. Run the backend by switching to the `backend` folder and use
```
cargo run
```
Or you can also do it in one-shot with
```
(cd backend && cargo run)
```
A rocket will be launched at `127.0.0.1:8000`, opening that URL is not necessary (there is nothing at that URL), the URL is used to serve HTTP `POST` and `GET` requests

### To Run CLI version of the Game
We also made an limited-feature, untested Command Line Interface(CLI) version of Connect 4 and TOOT-and-OTTO, if you don't want to install all the software to run `frontend` and `backend`, you can just run the CLI version directly to play the game by switching to the `model` folder and use:
```
cargo run
```
Again you can also do it in one-shot with
```
(cd model && cargo run)
```
