# TEC GTFS

THIS IS NOT RELATED TO THE TEC (OTW) IN ANY WAY. THIS IS A PERSONAL PROJECT.

## What is this?

This is a simple GTFS proxy to get data of any line of the TEC (OTW) network.

## Usage :

Download the GTFS file from LETEC.
Then:

```bash
$ cargo run --release
```

By default it will listen on port 3006.


## Linked projects

- [tec-fetcher](https://github.com/cK0nrad/tec-fetcher) 
    - Used to fetch the data from the TEC API and transmit it to websockets
- [tec-gtfs](https://github.com/cK0nrad/tec-gtfs)
    - Used to get data from the GTFS (meant to be merged in the fetcher)