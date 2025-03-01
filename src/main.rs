#[macro_use]
extern crate log;

use std::future::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

use tokio::io::{AsyncReadExt, AsyncWriteExt, Stdin, Stdout};

#[tokio::main]
async fn main() {
    real_main().await;
}

fn real_main() -> impl Future<Output = ()> {
    env_logger::init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    demo_server(stdin, stdout)
}

async fn demo_server(mut stdin: Stdin, mut stdout: Stdout) {
    let mut buffer = Vec::new();

    match stdin.read_buf(&mut buffer).await {
        Ok(a) => {
            error!("{:?}", a);
            match stdout.write_all(&buffer).await {
                Ok(_) => {}
                Err(e) => {
                    error!("{}", e.to_string());
                }
            };
        }
        Err(e) => {
            error!("{}", e.to_string());
        }
    };
}

#[wasm_bindgen(start)]
pub fn start() {
    spawn_local(real_main());
}
