# R CMD check results

0 errors | 0 warnings | 1 note

* Fix the date handling issue when the date is integer type.

## Note 1

* checking installed package size ... NOTE
  installed size is  7.9Mb
  sub-directories of 1Mb or more:
    libs   7.8Mb

The size of the package can't be reduced further as it has to bundle
all the Rust cates dependencies to avoid the downloading during
installation issue.
