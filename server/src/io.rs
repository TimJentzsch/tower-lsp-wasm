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
    fn once(event: &str, cb: &dyn Fn());
    #[wasm_bindgen(js_namespace = ["process", "stdin"])]
    fn read() -> Option<String>;
}

impl AsyncRead for Stdin {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        mut buf: &mut [u8],
    ) -> Poll<Result<usize>> {
        let mut res = "".to_owned();

        // Try to read as much from stdin as possible
        while let Some(s) = read() {
            res += &s;
        }

        if res.len() > 0 {
            // We read something, write it to the buffer
            write!(buf, "{}", res).unwrap();
            Poll::Ready(Ok(res.len()))
        } else {
            // There's nothing to read yet
            // Listen on stdin for something to become available
            let cb = || {
                cx.waker().wake_by_ref();
            };
            // We must use `stdin.once` instead of `stdin.on`.
            // The closure is allocated on the stack and can only be called once.
            // See https://rustwasm.github.io/docs/wasm-bindgen/reference/passing-rust-closures-to-js.html#stack-lifetime-closures
            once("readable", &cb);
            Poll::Pending
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
