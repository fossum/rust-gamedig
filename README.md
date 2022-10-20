# rust-gamedig
rust-GameDig is a game server/services query library, capable of querying the status of many games/services, this library brings what [node-GameDig](https://github.com/gamedig/node-gamedig) does, to pure Rust!  

MSRV is `1.58.1` and the code is cross-platform.

## Games/Protocols List
To see the supported (or the planned to support) games/protocols, see [GAMES](GAMES.md) and [PROTOCOLS](PROTOCOLS.md) respectively.

## Usage
Just pick a game, provide the ip and the port (can be optional) then query on it.
```rust
use gamedig::games::tf2;

fn main() {
    let response = tf2::query("91.216.250.10", None); //or Some(27015), None is the default protocol port
    match response {
        Err(error) => println!("Couldn't query, error: {error}"),
        Ok(r) => println!("{:?}", r)
    }
}
```
To see more examples, see the [examples](examples) folder.

## Documentation
The documentation is available at [docs.rs](https://docs.rs/gamedig/latest/gamedig/).  
Curious about the history and what changed between versions? you can see just that in the [CHANGELOG](CHANGELOG.md) file.

## Contributing
If you want see your favorite game/service being supported here, open an issue and I'll prioritize it! (or do a pull request if you want to implement it yourself)
