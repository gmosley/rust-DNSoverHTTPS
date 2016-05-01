# rust-DNSoverHTTPS

rust-DNSoverHTTPS is a DNS Server written in rust that uses Google's [DNS-over-HTTPS](https://developers.google.com/speed/public-dns/docs/dns-over-https) API for queries.

Developed by David Cao and Graham Mosley as a final project for [CIS 198](http://cis198-2016s.github.io/).

### Setup
rust-DNSoverHTTPS uses rust-nightly. At the time of writing, serde did not compile on nightly. It was testing using nightly-2016-04-17.

```
git clone https://github.com/gmosley/rust-DNSoverHTTPS.git
cd rust-DNSoverHTTPS
cargo build --release
sudo cargo run
```

Documentation can be found at: http://gmosley.github.io/rust-DNSoverHTTPS

Documentation of our rust-dns-parser fork can be found at: http://david-cao.github.io/rustdocs/dns_parser/
