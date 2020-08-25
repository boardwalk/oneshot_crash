#![feature(async_closure)]
use anyhow::Error;
use futures_util::future::{pending, BoxFuture};
use futures_util::stream::{FuturesUnordered, StreamExt as _};

type Futures<'a> = FuturesUnordered<BoxFuture<'a, Result<(), Error>>>;

fn main() -> Result<(), Error> {
    smol::run(main_inner())
}

async fn main_inner() -> Result<(), Error> {
    let mut futures = Futures::new();
    let (sender, receiver) = async_oneshot::oneshot::<()>();

    let keepalive = async move || {
        let _sender = sender;
        pending::<()>().await;
        Ok(())
    };


    let wait_for_signal = async move || {
        let _ = receiver.await;
        Ok(())
    };


    let complete = async || {
        Ok(())
    };

    futures.push(Box::pin(keepalive()));
    futures.push(Box::pin(wait_for_signal()));
    futures.push(Box::pin(complete()));
    futures.next().await.unwrap()
}

// % gdb -ex run -args target/debug/oneshot_crash
// (gdb) bt
// #0  0x0000555555583729 in core::task::wake::Waker::wake_by_ref (self=0x5555556b4f58)
//     at /home/boardwalk/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/task/wake.rs:256
// #1  0x000055555557dcf7 in <async_oneshot::receiver::Receiver<T> as core::ops::drop::Drop>::drop (self=0x5555556b5020)
//     at /home/boardwalk/.cargo/registry/src/github.com-1ecc6299db9ec823/async-oneshot-0.3.1/src/receiver.rs:70
// #2  0x000055555557b1b2 in core::ptr::drop_in_place () at /home/boardwalk/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/ptr/mod.rs:184
// #3  0x0000555555579a76 in core::ptr::drop_in_place () at src/main.rs:24
// #4  0x000055555557ad0e in core::ptr::drop_in_place () at /home/boardwalk/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/ptr/mod.rs:184
// #5  0x000055555557ae0c in core::ptr::drop_in_place () at /home/boardwalk/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/ptr/mod.rs:184
// #6  0x0000555555578c5e in core::ptr::drop_in_place () at /home/boardwalk/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/ptr/mod.rs:184
// #7  0x0000555555578ca5 in core::ptr::drop_in_place () at /home/boardwalk/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/ptr/mod.rs:184
// #8  0x000055555558207d in futures_util::stream::futures_unordered::FuturesUnordered<Fut>::release_task (self=0x7fffffffc868, task=...)
//     at /home/boardwalk/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-util-0.3.5/src/stream/futures_unordered/mod.rs:289
// #9  0x0000555555578681 in <futures_util::stream::futures_unordered::FuturesUnordered<Fut> as core::ops::drop::Drop>::drop (self=0x7fffffffc868)
//     at /home/boardwalk/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-util-0.3.5/src/stream/futures_unordered/mod.rs:589
// #10 0x000055555557b262 in core::ptr::drop_in_place () at /home/boardwalk/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/ptr/mod.rs:184
// #11 0x0000555555587609 in oneshot_crash::main_inner::{{closure}} () at src/main.rs:37
// #12 0x000055555558f149 in <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll (self=..., cx=0x7fffffffc8c8)
//     at /home/boardwalk/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/future/mod.rs:74
// #13 0x00005555555744d0 in smol::run::{{closure}} () at /home/boardwalk/.cargo/registry/src/github.com-1ecc6299db9ec823/smol-0.3.3/src/lib.rs:161
// #14 0x000055555558f1f9 in <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll (self=..., cx=0x7fffffffc8c8)
//     at /home/boardwalk/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/future/mod.rs:74
// #15 0x000055555558897e in async_executor::LocalExecutor::run::{{closure}} () at /home/boardwalk/.cargo/registry/src/github.com-1ecc6299db9ec823/async-executor-0.1.2/src/lib.rs:339
// #16 0x000055555557e4c6 in scoped_tls::ScopedKey<T>::set (self=0x555555696348 <async_executor::LOCAL_EX>, t=0x7fffffffd890, f=...)
//     at /home/boardwalk/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
// #17 0x00005555555888bd in async_executor::LocalExecutor::run (self=0x7fffffffd890, future=...)
//     at /home/boardwalk/.cargo/registry/src/github.com-1ecc6299db9ec823/async-executor-0.1.2/src/lib.rs:337
// #18 0x0000555555574819 in smol::run::{{closure}}::{{closure}} () at /home/boardwalk/.cargo/registry/src/github.com-1ecc6299db9ec823/smol-0.3.3/src/lib.rs:179
// #19 0x000055555557e5fb in scoped_tls::ScopedKey<T>::set (self=0x555555696338 <async_executor::EX>, t=0x7fffffffd888, f=...)
//     at /home/boardwalk/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
// #20 0x000055555558910f in async_executor::Executor::enter (self=0x7fffffffd888, f=...) at /home/boardwalk/.cargo/registry/src/github.com-1ecc6299db9ec823/async-executor-0.1.2/src/lib.rs:151
// #21 0x0000555555574709 in smol::run::{{closure}} () at /home/boardwalk/.cargo/registry/src/github.com-1ecc6299db9ec823/smol-0.3.3/src/lib.rs:179
// #22 0x000055555556ec85 in <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once (self=..., _args=())
//     at /home/boardwalk/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/panic.rs:318
// #23 0x00005555555895b6 in std::panicking::try::do_call (data=0x7fffffffce50 "\210\330\377\377\377\177\000")
//     at /home/boardwalk/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/panicking.rs:342
// #24 0x000055555558976d in __rust_try ()
// #25 0x00005555555894b0 in std::panicking::try (f=...) at /home/boardwalk/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/panicking.rs:319
// #26 0x000055555556ecfd in std::panic::catch_unwind (f=...) at /home/boardwalk/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/panic.rs:394
// #27 0x0000555555593b06 in easy_parallel::Parallel<T>::finish (self=..., f=...) at /home/boardwalk/.cargo/registry/src/github.com-1ecc6299db9ec823/easy-parallel-3.1.0/src/lib.rs:227
// #28 0x000055555557423e in smol::run (future=...) at /home/boardwalk/.cargo/registry/src/github.com-1ecc6299db9ec823/smol-0.3.3/src/lib.rs:177
// #29 0x0000555555589cd8 in oneshot_crash::main () at src/main.rs:9