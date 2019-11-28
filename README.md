# otus_backend
The backend of the Otus Engine. With the all new Rocket.rs!

## How do I run it?
It's simple! All you need to do is fill out two different variables.
In Rocket.toml, input the URI for your postgres database. If you haven't created the required tables to run the backend, run the create_db.sh script (but fill out the URI in the script first!) to do that. Then, go to the file src/views/create_image and put in you Recaptcha secret. (This is in order to prevent bots from making images). After that, run `cargo build --release` and the binary will be generated! Alternatively, you can download the binary from the releases page if you don't have Rust. Run the binary in the source code directory and enjoy!
