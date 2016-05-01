# CIS 198 Final Report

rust-DNSoverHTTPS is a DNS Server written in rust that uses Google's [DNS-over-HTTPS](https://developers.google.com/speed/public-dns/docs/dns-over-https) API for queries. It was developed by David Cao and Graham Mosley as a final project for [CIS 198](http://cis198-2016s.github.io/). This project consists of the actual rust-DNSoverHTTPS server and a fork of [dns-parser](https://github.com/david-cao/dns-parser).

Documentation for the project is located at [http://gmosley.github.io/rust-DNSoverHTTPS](http://gmosley.github.io/rust-DNSoverHTTPS).

Documentation for our fork of dns-parser is located at [http://david-cao.github.io/rustdocs/dns_parser/](http://david-cao.github.io/rustdocs/dns_parser/).

For instructions on building/running rust-DNSoverHTTPS see the [README](https://github.com/gmosley/rust-DNSoverHTTPS/blob/master/README.md).

## Approximate time spent
We feel that over the past 3 weeks, a significant amount of time was spent on the project. Since this was also our first time working with the DNS, we also spent a fair amount of time understanding the protocol.

While we didn't keep track of the exact time spent on the project, we estimate that we spent a combined 15 hours for the first two weeks and a combined 20 hours for the final week.

## Accomplishments
We were able to create a working DNS server with almost the same average response time as traditional DNS servers for the most common queries! See the results section for more information.

## Components, structure and design decisions
rust-DNSoverHTTPS handles a standard DNS query in the following steps:

1. The server listens for incoming UDP packets on port 53. When a packet is received, a new thread is spawned.
2. The packet is parsed into a DNS packet using `dns-parser`.
3. If the parsing is successful, a HTTPS request is constructed and sent using `hyper`.
4. The response is deserialized using serde.
5. A DNS answer packet is constructed and sent back to the requestor. 

As mentioned above, our work was split between our fork of dns-parser and the actual server itself. This gave us experience working with an already existing codebase and choosing the design for our own code.

### dns-parser fork
dns-parser was chosen to parse DNS packets. Unfortunately, while library was great for parsing packets, it had very little support for building packets. In our fork of dns-parser, the majority of our work was in [`builder.rs`](http://david-cao.github.io/rustdocs/dns_parser/builder/struct.Builder.html). We implemented many functions including `new_response` and `add_question`. We also implemented DNS packet compression.

### High level overview of components
* `main.rs` - sends and receives DNS UDP packets
* `structs.rs` - structs used for serde deserialization of Google API responses.
* `errors.rs` - enum of possible errors that can occur

## Benchmarks/Results
We found that on average our DNS server had under 100 ms response time for queries when built with the `--release` flag. This is only slightly slower than the mean response time of the University of Pennsylvania's DNS Servers. Our server also has the security benefits of HTTPS over DNS.

### Benchmarking was done using [namebench](https://github.com/catap/)

rust-DNSoverHTTPS, Google Public DNS, and the University of Pennsylvania's DNS were tested. Benchmarking was run on OSX using AirPennNet. The DNS cache was cleared before benchmarking.

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

#### % of requests fulfilled in duration (ms)
![](https://raw.githubusercontent.com/gmosley/rust-DNSoverHTTPS/master/results.png)


## Limitations
rust-DNSoverHTTPS is not a full DNS Server, but rather a proof of concept of using DNS-over-HTTPS.

rust-DNSoverHTTPS supports the following record types:

* A
* AAAA
* CNAME
* PTR

While rust-DNSoverHTTPS only supports 4 record types (~10% of the protocol) there are no noticeable problems when browsing the web. 

Additionally rust-DNSoverHTTPS will probably fail most DNS health checks since queries like hostname.bind and id.server are unsupported.

## Postmortem
We had a great time working on rust-DNSoverHTTPS. We gained a huge amount experience with Rust and the DNS protocol.

### What went well
It worked! There were a few times during the project that we were not sure if we could get everything working. For example, when we first implemented A and CNAME records, we could get extremely simple websites to load, but they took almost 10 seconds to do so. We realized that this was because we panic'd on empty responses and/or the website that was loaded was loading more things in turn, such as scripts or fonts hosted at Google. Fortunately, we were able to fix this and got every working.

### What could have gone better
The main goal of our project was to support the minimum amount of features of the protocol needed to browse the web using our DNS server. We assumed that we would only need to implement A and CNAME record types. Because of this we didn't implement a system that made it easy to support various record types. This made adding later types more difficult. Specifically, in `add_answer` we pass in a vec of bytes, which we compute ourselves based on the type. In the future, we could modify `dns_parser::rrdata` to use it for record type serialization (instead of just deserialization).
