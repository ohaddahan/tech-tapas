+++
title = 'SQL the right way'
date = 2024-01-28T09:22:33+02:00
draft = false
+++

![sql meme](https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/1eaf6d57-3a83-4476-c7fe-e20feb5cce00/public)

## Background

`SQL` is a fundamental building block of almost any application.
It's the most popular database query language.

There are 2 popular approaches to using `SQL` in applications:

1. **ORM** (Object-Relational Mapping) - A wrapper around `SQL` that abstracts the underlying interface from the user.
2. **Raw SQL** - Writing `SQL` queries directly in the application code.

We'll discuss `#2` and how to do it the right way.

## [sqlx](https://github.com/launchbadge/sqlx)

[sqlx](https://github.com/launchbadge/sqlx) is a modern `SQL` library for `rust` with support for `async`.
But the feature that is truly a game changer is the `compile` time verification and `type casting`.

### Compile time verification

In [sqlx](https://github.com/launchbadge/sqlx) if you use [query!](https://docs.rs/sqlx/latest/sqlx/macro.query.html)
or [query_as!](https://docs.rs/sqlx/latest/sqlx/macro.query_as.html) (and the rest of this `macro` family)
it will verify your `SQL` during  `cargo build` phase.

For example, let's assume the following `SQL` scheme:

```sql
CREATE TABLE users
(
    id       uuid PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
);
```

If we want to fetch a `User` by `username` we can use the following`:

```rust
match sqlx::query_as!(
    User,
    r#"
    SELECT id, username
    FROM users
    WHERE username = $1
    LIMIT 1"#,
   username 
)
```

```rust
#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
}
```

* If we pass `username` that isn't a `String` it will fail during `cargo build` phase.
* If we have a `typo` in one of the fields it will fail during `cargo build` phase.
* If we did some `migration` and forgot to update the `query` it will fail during `cargo build` phase.

I can't stress enough how much time and effort this feature saves. It helped me tremendously, especially
on `refactoring`.

### Type casting

Let's add a `password` field to our `User`:

```sql
CREATE TABLE users
(
    id       uuid PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
);
```

```rust
#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(skip)]
    pub password: Secret<String>,
}
```

Obviously, we would like as little exposure to `password` as possible.
To prevent leakage of any sorts.

In `rust` side, we can define a `type` that will prevent any `logger` to accidentally print it.
And will require `explicit` call to `expose` the `secret`.

```rust
pub struct Secret<T>(T)
    where
        T: Clone;

impl<T> Clone for Secret<T>
    where
        T: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Default for Secret<T>
    where
        T: Clone + Default,
{
    fn default() -> Self {
        Self(T::default())
    }
}

impl<T> Display for Secret<T>
    where
        T: Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[hidden]")
    }
}

impl<T> Debug for Secret<T>
    where
        T: Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[hidden]")
    }
}
```

But how do we combine `secret` with `sqlx`?

```rust
match sqlx::query_as!(
    User,
    r#"
    SELECT id, username, password as "password: Secret<String>"
    FROM users
    WHERE username = $1
    LIMIT 1"#,
   username 
)
```

Here `password as "password: Secret<String>"` is where the `magic` happens.
This tells `sqlx` to `cast` the `password` to `Secret<String>`.
By doing so, other than `query logs` (which you can mute),
`password` will be `hidden` from any `logger`.

This example shows how `sqlx` helps bring `rust` powerful `type` system into `SQL`.
All during `cargo build` phase, with little runtime surprises.

## References

* [sqlx](https://github.com/launchbadge/sqlx)
* [Raw SQL in Rust with SQLx](https://www.shuttle.rs/blog/2023/10/04/sql-in-rust)