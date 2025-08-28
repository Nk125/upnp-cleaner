# UPnP Cleaner

Unmap ports from the UPnP table from the router

This uses the [IGD-next crate]

[IGD-next crate]: https://docs.rs/igd-next/

## Usage

Run the UPnP cleaner with the desired external port and protocol to unmap

```bash
# Here's cleaning a tcp map with port 443
upnp-cleaner -n tcp -p 443

# Or an udp map
upnp-cleaner -n udp -p 1234
```

## Compilation

Download and install rust from [rustup.sh](rustup.sh)

Also install [`git`](https://git-scm.com/)

Then with `cargo` and `git` available, run in your terminal:

```bash
# Clone the repository
git clone https://github.com/Nk125/upnp-cleaner

# Build the program
cargo build --release

# Or run it directly
cargo run --release -- -n tcp -p 443
```
