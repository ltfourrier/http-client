# HTTP Client

This repository contains the code for the `http-client` crate, which serves as an abstraction over actual HTTP clients used by LTFNet and/or Tiwind Software projects.

**As such, it is tailored for my personal use and should probably not be used by projects outside LTFNet/TWS.**

You're still welcome to use this code, copy it, modify it, and ship it however you like, but you're on your own if you decide to do so.

## Crates available

### http-client

This crate contains the HTTP client abstraction:
- A `HttpClient` trait that can be implemented to provide a usable HTTP client.
- A `HttpRequest` structure that is used by the `HttpClient` trait to represent an HTTP request, and a associated `HttpRequestBuilder` that provides a builder-like interface to create such requests.
- A `HttpResponse` structure that is returned by the `HttpClient` trait when a request is completed.
- And finally a `HttpError` type to represent potential errors that can occur during HTTP requests.

All these types are documented (albeit a bit roughly) and can be used along with an _implementation_, which is provided in this repository by the `http-client-*` crates.

There is also the `json` feature that can be enabled to add support for JSON serialization/deserialization using serde.

### http-client-hyper

This crate provides an implementation of the `HttpClient` trait using the [Hyper](https://hyper.rs/) library.

It supports HTTPS/TLS, although TLS support was not tested much.