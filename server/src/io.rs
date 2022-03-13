use std::{
    io::Result,
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

impl AsyncRead for Stdin {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize>> {
        todo!()
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
