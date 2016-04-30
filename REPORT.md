# CIS 198 Final Report

## Summary
rust-DNSoverHTTPS is a DNS Server written in rust that uses Google's [DNS-over-HTTPS](https://developers.google.com/speed/public-dns/docs/dns-over-https) API for queries. It was developed by David Cao and Graham Mosley as a final project for [CIS 198](http://cis198-2016s.github.io/). This project consists of the actual rust-DNSoverHTTPS server and a fork of [dns-parser](https://github.com/david-cao/dns-parser).

Documentation for the project is located at [http://gmosley.github.io/rust-DNSoverHTTPS](http://gmosley.github.io/rust-DNSoverHTTPS)

Documentation for our fork of dns-parser is located at [http://david-cao.github.io/rustdocs/dns_parser/](http://david-cao.github.io/rustdocs/dns_parser/)

For instructions on building/running rust-DNSoverHTTPS see the [README](https://github.com/gmosley/rust-DNSoverHTTPS/blob/master/README.md)

## Approximate time spent
We feel that over the 3 weeks, a significant amount of time was spent on the project. 
Since this was also our first time working with the DNS, we also spent a fair amount of time understanding the protocol.

## Accomplishments
We were able to create a working DNS server with almost the same average response time as traditonal DNS servers for a variety of standard queries.

## Components, structure, design decisions
A typical DNS query is handled in the following steps:

1. The server listens for incoming UDP packets on port 53. When a packet is received, a new thread is spawned.
2. The packet is parsed into a DNS packet using `dns-parser`.
3. If the parsing is successfull, a HTTPS request is constructed and sent using `hyper`.
4. The response is deserialized using serde.
5. A DNS answer packet is constructed and sent back to the requestor. 

dns-parser was chosen to parse DNS packets. Unfortunately, while library was great for parsing packets, it had very little support for building packets. In

## Benchmarks/Results
We found that our DNS server had ~150 ms response time for queries when built with the `--release` flag. This is only slightly slower than the average response time of the University of Pennsylvania's DNS Servers (TODO ms). Our server also had the benifits of HTTPS over DNS.

### Benchmarking was done using [namebench](https://github.com/catap/)

rust-DNSoverHTTPS, Google Public DNS, and the University of Pennsylvania's DNS were tested. Benchmarking was run on OSX using AirPennNet. The DNS cache was cleared before use.

```./namebench.py -H -q 500 127.0.0.1 8.8.8.8 128.91.49.1```

#### Fastest individual response:

| DNS Server        | ms      |
| ----------------  | ------- |
| Penn DNS          | 2.66790 |
| Google Public DNS | 5.82314 |
| rust-DNSoverHTTPS | 46.1969 |

#### Mean response:

| DNS Server        | ms    |
| ----------------  | ----- |
| Google Public DNS | 50.15 |
| Penn DNS          | 64.97 |
| rust-DNSoverHTTPS | 84.02 |

## Limitations
rust-DNSoverHTTPS is not a full DNS Server, but rather a proof of concept of using DNS-over-HTTPS. However, rust-DNSoverHTTPS works for enough DNS queries to browse the web. 

rust-DNSoverHTTPS supports the following record types:

* A
* AAAA
* CNAME
* PTR

Additionally rust-DNSoverHTTPS will probably fail most DNS health checks.

## Postmortem
We had a great time working on rust-DNSoverHTTPS.
