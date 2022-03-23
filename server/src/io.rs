use std::{
    io::{Result, Write},
    pin::Pin,
    str,
    task::{Context, Poll},
};

use futures::{AsyncRead, AsyncWrite};
use wasm_bindgen::prelude::*;

pub fn stdin() -> Stdin {
    Stdin
}

pub fn stdout() -> Stdout {
    Stdout
}

pub struct Stdin;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["process", "stdin"])]
    fn once(event: &str, closure: &Closure<dyn FnMut()>);
    #[wasm_bindgen(js_namespace = ["process", "stdin"])]
    fn read() -> Option<String>;
}

impl AsyncRead for Stdin {
    fn poll_read(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        mut buf: &mut [u8],
    ) -> Poll<Result<usize>> {
        let mut res = "".to_owned();

        loop {
            // Try to read as much from stdin as possible
            while let Some(s) = read() {
                res += &s;
            }

            if res.len() > 0 {
                // We read something, write it to the buffer and return
                write!(buf, "{}", res).unwrap();
                return Poll::Ready(Ok(res.len()));
            }
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["process", "stdout"])]
    fn write(s: &[u8]);
}
pub struct Stdout;

impl AsyncWrite for Stdout {
    fn poll_write(self: Pin<&mut Self>, _cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize>> {
        write(buf);
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<()>> {
        Poll::Ready(Ok(()))
    }
}
