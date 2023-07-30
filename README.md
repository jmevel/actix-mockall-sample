# Actix mockall sample

This sample application shows how to use [mockall](https://github.com/asomers/mockall) with [Actix](https://actix.rs).

An *Email Client* is taken as an example:

- A "real" email client is stored in Actix's Application Data when running the project.
- A mock is used instead of the real email client when running the integration test.
