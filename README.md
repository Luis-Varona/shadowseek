# shadowseek

![Version](https://img.shields.io/badge/version-v0.1.0--dev-orange)
![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-rebeccapurple)
[![Build Status](https://github.com/Luis-Varona/shadowseek/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/Luis-Varona/shadowseek/actions/workflows/rust.yml?query=branch%3Amain)

*ShadowSeek* is a CLI tool for near-duplicate detection in text files. Written in native Rust, it offers fast execution and low memory overhead, with no dependencies on external runtime environments. Using the [`textract`](https://crates.io/crates/textract), [`rtf-parser`](https://crates.io/crates/rtf-parser), and [`epub`](https://crates.io/crates/epub) crates to parse various text file formats, it uses SimHash to quickly filter out highly dissimilar documents then applies a more sophisticated MinHash algorithm to identify near-duplicates with high accuracy.

Inspired by [Dr. Pawe&#322; Mandera](https://github.com/pmandera)'s near-duplicate detection tool [*Duometer*](https://github.com/pmandera/duometer), *ShadowSeek* aims to provide a more lightweight and efficient alternative. Development in Rust allows users to run a precompiled binary without needing to install a Java runtime environment; as an added bonus, this also reduces startup time and memory usage. The inclusion of SimHash as a first-pass filter also facilitates faster elimination of dissimilar documents, minimizing the number of comparisons performed in the more computationally expensive MinHash stage.

Inspired by [Dr. Pawe&#322; Mandera](https://github.com/pmandera)'s near-duplicate detection tool [*Duometer*](https://github.com/pmandera/duometer), *ShadowSeek* aims to provide a more lightweight and efficient alternative. Development in Rust allows users to run a precompiled binary without needing to install a Java runtime environment; as an added bonus, this also reduces startup time and memory usage. The inclusion of SimHash as a first-pass filter also facilitates faster elimination of dissimilar documents, minimizing the number of comparisons performed in the more computationally expensive MinHash stage.

**(CURRENTLY UNDER DEVELOPMENT)**
