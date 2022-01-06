test_that("bond works", {
  ymd <- function(...) as.Date(as.character(list(...)))
  expect_error(
    bond_result(ymd("2021-01-01", "2021-02-01"), ymd("2025-01-01", "2025-02-01"), 100.0, 0.05, 0L, ymd("2022-01-01", "2022-02-01"), 100)
  )
  expect <- data.frame(
    ytm = c(0.0454848062096707, 0.0299794780750772),
    macd = c(3.00036561713138, 7.23139084505863),
    modd = c(2.8721026917994, 7.02455669824076)
  )
  out <- bond_result(ymd("2021-01-01", "2021-02-01"), ymd("2025-01-01", "2030-02-01"), c(100.0, 100.0), c(0.05, 0.03), c(0L, 1L), ymd("2022-01-01", "2022-02-01"), c(100, 100))
  expect_equal(out, expect)

  out <- bond_cf(ymd("2026-01-01", "2021-02-01"), ymd("2025-01-01", "2030-02-01"), c(100.0, 100.0), c(0.05, 0.03), c(0L, 1L))
  expect <- data.frame(ID = 2L, DATE = as.Date(sprintf("%s-02-01", 2022:2030)), CF = c(rep(3, 8), 103))
  expect_equal(out, expect)

  out <- bond_result(ymd("2021-01-01", "2021-02-01"), ymd("2025-01-01", "2030-02-01"), c(100.0, NA), c(0.05, 0.03), c(0L, 1L), ymd("2022-01-01", "2022-02-01"), c(100, 100))
  na_out <- c(NA_real_, NA_real_, NA_real_)
  expect_equal(as.double(out[2, ]), na_out)
  expect_equal(
    as.double(bond_result(ymd("2021-01-01"), ymd("2020-01-01"), 100.0, 0.05, 1L, ymd("2020-01-01"), 100.0)), na_out
  )
  expect_equal(
    as.double(bond_result(ymd("2018-01-01"), ymd("2020-01-01"), 100.0, 0.05, 1L, ymd("2022-01-01"), 100.0)), na_out
  )
  expect_equal(
    as.double(bond_result(ymd("2018-01-01"), ymd("2020-01-01"), 100.0, 0.05, 3L, ymd("2019-01-01"), 100.0)), na_out
  )
})
