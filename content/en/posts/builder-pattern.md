+++
title = 'Builder Pattern'
date = 2024-01-28T09:22:33+02:00
draft = false
+++

![Builder Pattern](https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/56f3edee-d864-4cc0-160d-095b05639f00/public)

## Background

Often, we need to create complex objects with many parameters, some mandatory, some optional.

The Builder pattern helps us create them step by step and expose a nice friendly API to the client.

## Common use cases

`HTTP` clients often use the builder pattern to create requests.

For example [reqest](https://github.com/seanmonstar/reqwest) is a popular Rust HTTP client that uses the builder
pattern.

```rust
let echo_json: serde_json::Value = reqwest::Client::new()
.post("https://jsonplaceholder.typicode.com/posts")
.json( & serde_json::json!({
            "title": "Reqwest.rs",
            "body": "https://docs.rs/reqwest",
            "userId": 1
        }))
.send()
.await?
.json()
.await?;
```

In this example we can see the following steps:

1. Create a new client.
2. Create a new `POST` request.
3. Set the request body to a JSON object.
4. Send the request.
5. `await` the response.
6. Parse the response as `JSON`.

This is a very clear and concise way to describe the flow and any reader will understand with ease.

## How to build your own

1. Define your `struct` and `builder`.

```rust
#[derive(Debug)]
struct House {
    walls: i32,
    doors: i32,
    windows: i32,
}

#[derive(Debug)]
struct HouseBuilder {
    walls: i32,
    doors: i32,
    windows: i32,
    status: u8,
}
```

2. Implement the `constructor` to the `builder`.

```rust
pub fn new() -> HouseBuilder {
    HouseBuilder {
        walls: 0,
        doors: 0,
        windows: 0,
        status: 0b0000_0000,
    }
}
```

3. Implement the `setter` methods.

```rust
 pub fn walls(&mut self, walls: i32) -> &mut Self {
    self.walls = walls;
    self.status = self.status | 0b0000_0001;
    self
}

pub fn doors(&mut self, doors: i32) -> &mut Self {
    self.doors = doors;
    self.status = self.status | 0b0000_0010;
    self
}

pub fn windows(&mut self, windows: i32) -> &mut Self {
    self.windows = windows;
    self.status = self.status | 0b0000_0100;
    self
}
```

4. Finally implement the `build` method.

```rust
pub fn build(&self) -> Result<House, HouseBuilderError> {
    if self.status != 0b0000_0111 {
        return Err(HouseBuilderError);
    }
    Ok(House {
        walls: self.walls,
        doors: self.doors,
        windows: self.windows,
    })
}
```

To the keen reader, you may have noticed the `status` field.
This is a `bitmask` that we use to keep track of the fields that have been set.
Which allows us to use `Result` and let the client know if there is any issue.

```rust
fn main() {
    let house1 = HouseBuilder::new().doors(2).windows(4).walls(1).build();
    assert!(house1.is_ok());
    if let Ok(house1) = house1 {
        println!("house1 {:#?}", house1);
        assert_eq!(2, house1.doors);
        assert_eq!(4, house1.windows);
        assert_eq!(1, house1.walls);
    }
    let house2 = HouseBuilder::new().windows(4).walls(3).doors(1).build();
    assert!(house2.is_ok());
    if let Ok(house2) = house2 {
        println!("house2 {:#?}", house2);
        assert_eq!(1, house2.doors);
        assert_eq!(4, house2.windows);
        assert_eq!(3, house2.walls);
    }
    let house3 = HouseBuilder::new().windows(4).build();
    assert!(house3.is_err());
    if let Err(e) = house3 {
        println!("house3 {:#?}", e);
    }
}
```

## References

- [builder code](https://github.com/ohaddahan/tech-tapes/tree/master/examlpes/builder/src/main.rs)
- [Builder Pattern](https://en.wikipedia.org/wiki/Builder_pattern)
- [Builder](https://refactoring.guru/design-patterns/builder)