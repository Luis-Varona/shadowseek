# shadowseek

![Version](https://img.shields.io/badge/version-v0.1.0--dev-orange)
![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-rebeccapurple)
[![Build Status](https://github.com/Luis-Varona/shadowseek/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/Luis-Varona/shadowseek/actions/workflows/rust.yml?query=branch%3Amain)

*ShadowSeek* is a CLI tool for near-duplicate detection in text files. Written in native Rust, it offers fast execution and low memory overhead, with no dependencies on external runtime environments. Using the [`textract`](https://crates.io/crates/textract) crate to extract text in various file formats, it uses SimHash to quickly filter out clearly dissimilar documents then applies a more sophisticated MinHash algorithm to identify near-duplicates with high accuracy.

**(CURRENTLY UNDER DEVELOPMENT)**
