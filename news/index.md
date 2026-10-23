# Changelog

## fcl 0.1.5

CRAN release: 2026-08-22

- Restore CRAN R C API compliance by upgrading the `extendr` Rust
  crates.
- Require Rust 1.71 or later and validate the Rust toolchain during
  configuration.
- Build the bundled Rust dependencies offline in a single
  package-install build.

## fcl 0.1.4

CRAN release: 2025-04-15

- Fix the calling non-API entry points issue by upgrading the rust
  crates.

## fcl 0.1.3

CRAN release: 2024-11-26

- Fix the date handling issue when the date is integer type.

## fcl 0.1.2

CRAN release: 2024-08-23

- Added rustc version reporting in the installation log as per CRAN
  policy.

## fcl 0.1.1

CRAN release: 2024-08-23

- Fix the calling non-API entry points issue.

## fcl 0.1.0

CRAN release: 2023-10-13

- Added a `NEWS.md` file to track changes to the package.
- `bond_result()` supports vector inputs.
- `bond_result()` tries to return `NA` for unexpected input or when
  calculating IRR fails.
- Implement `bond_cf()`, which returns forecast bond cash flows.
- Implement `rtn()` which allows for fast TWRR or Modified Return.
- Better handle a zero denominator in return calculations.
- Dietz return now uses the same day count handling as TWRR, which is
  assuming positive cash flow occurs at the BOP of the day while
  negative cash flow occurs at the EOP.
- Use
  [`fixed_bond()`](https://shrektan.github.io/fcl/reference/fixed_bond.md)
  to create bond objects with `$cf()` and `$ytm_dur()` methods.
