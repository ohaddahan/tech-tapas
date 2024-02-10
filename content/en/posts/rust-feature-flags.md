+++
title = 'Rust Feature Flags'
date = 2024-02-10T09:22:33+02:00
draft = false
+++

![Rust Feature Flags](https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/884d21a8-f178-42bc-b313-3def88a8b100/public)

## Background

Recently while working on a project, I stumbled upon an annoying situation.
The `API` for a `test` and `production` client were different.
The `production` client was `immutable` while the `test` client was `mutable`.
They also had different methods to implement the same functionality.

I needed a way to use the same code for both clients.

## Road to victory

### [Enum Variants](https://doc.rust-lang.org/reference/items/enumerations.html)

So I created an `enum` with two variants, `Test` and `Production`.

```rust
enum Client {
    Test(TestClient),
    Production(ProductionClient),
}
```

And the usage was like this:

```rust
async fn boo(client: &Client) {
    match client {
        Client::Test(client) => {
            client.do_something_1();
        }
        Client::Production(client) => {
            client.do_something_2();
        }
    }
}
```

Good start, but this didn't work since `TestClient` needed to be `mutable`.
Ok, let's pass in a mutable reference.

```rust
async fn boo(client: &mut Client) {
    match client {
        Client::Test(client) => {
            client.do_something_1();
        }
        Client::Production(client) => {
            client.do_something_2();
        }
    }
}
```

### [Arc<Mutex<T>>](https://doc.rust-lang.org/book/ch16-03-shared-state.html)

That works, almost. I was using this across multiple `server` handlers, background `workers` etc.
Ok let's slap an `Arc<Mutex<Client>>` on it.

```rust
async fn boo(client: Arc<Mutex<Client>>) {
    let client = client.lock().await;
    match client {
        Client::Test(client) => {
            client.do_something_1();
        }
        Client::Production(client) => {
            client.do_something_2();
        }
    }
}
```

This works, but now every request to the client is `sequential`, for `test` I don't mind.
But for `production` I need `concurrency`, especially since the `production` client `immutable`, there is no
justification for this.

### [#[cfg] macro](https://doc.rust-lang.org/book/ch11-03-test-organization.html)

So now I tried using `#[cfg(test)]`.

```rust
#[cfg(test)]
async fn boo(client: Arc<Mutex<TestClient>>) {
    let client = client.lock().await;
    client.do_something_1();
}

#[cfg(not(test))]
async fn boo(client: Arc<ProductionClient>) {
    client.do_something_2();
}
```

This was good, until I tried running
my [integration](https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests) tests.
In `rust` , [integration](https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests) don't run
in `#[cfg(test)]` mode.
So back to the drawing board.

### [Feature Flags](https://doc.rust-lang.org/cargo/reference/features.html)

So instead of using the standard `#[cfg(test)]`, I used `feature flags`.

```toml
[features]
test-client = []
```

And the code changed to:

```rust
#[cfg(feature = "test-client")]
async fn boo(client: Arc<Mutex<TestClient>>) {
    let client = client.lock().await;
    client.do_something_1();
}

#[cfg(not(feature = "test-client"))]
async fn boo(client: Arc<ProductionClient>) {
    client.do_something_2();
}
```

So in regular mode, `test-client` feature is not enabled.
And I just needed to add `--features test-client` to my `cargo test` commands.
While this looks dirty since I have duplicate code, if I would use the same `method`
it would still need two different implementations due to differences in `clients`.

## References

* [GitHub Example Code](https://github.com/ohaddahan/tech-tapas/tree/master/examlpes/cfg-features/src/main.rs)
* [Shared State Concurrency](https://doc.rust-lang.org/book/ch16-03-shared-state.html#shared-state-concurrency)
* [Test Organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html#test-organization)
* [Features](https://doc.rust-lang.org/cargo/reference/features.html)