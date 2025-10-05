# sixx-fs

A rust gui project powered by [iced](https://github.com/iced-rs/iced).
`fs` means file scanner and `sixx` means me.

Still in development.

## Features

- Scan a directory recursively for a structural record
  - includes information for all files inside
  - stores in a json file
  - supports symlinks (but won't jump out while scanning)
- (WIP) Scan a single file and add it to a list record
- View the records

## Specification

- Stores timestamp as i64 nano seconds
  - only support the dates in ±290 years from the unix epoch
  - if you come from 2260, just change i64 to i128 and ask the crate `chrono` to support it
