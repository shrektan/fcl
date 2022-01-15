test_that("bond works", {
  ymd <- function(...) ymd::ymd(unlist(list(...)))
  out <- bond_result(ymd("2021-01-01", "2021-02-01"), ymd("2025-01-01", "2025-02-01"), 100.0, 0.05, 0L, ymd("2022-01-01", "2022-02-01"), 100)
  expect_equal(as.double(out[1,]), as.double(out[2,]))
  expect <- data.frame(
    YTM = c(0.0455272763905981, 0.03),
    MACD = c(3.0, 7.23028295522156),
    MODD = c(2.86936559941372, 7.01969218987131)
  )
  out <- bond_result(ymd("2021-01-01", "2021-02-01"), ymd("2025-01-01", "2030-02-01"), 100.0, c(0.05, 0.03), c(0L, 1L), ymd("2022-01-01", "2022-02-01"), 100)
  expect_equal(out, expect)

  out <- bond_cf(ymd("2026-01-01", "2021-02-01"), ymd("2025-01-01", "2030-02-01"), c(100.0, 100.0), c(0.05, 0.03), c(0L, 1L), ymd("2026-01-01", "2021-02-01"))
  expect <- data.frame(ID = 2L, DATE = as.Date(sprintf("%s-02-01", 2022:2030)), COUPON = rep(3, 9), REDEM = c(rep(0, 8), 100))
  expect_equal(out, expect)

  out <- bond_cf(ymd("2021-02-01"), ymd("2030-02-01"), c(100.0), c(0.03), c(1L), ymd("2024-01-01"))
  expect <- data.frame(ID = 1L, DATE = as.Date(sprintf("%s-02-01", 2024:2030)), COUPON = rep(3, 7), REDEM = c(rep(0, 6), 100))
  expect_equal(out, expect)

  out <- bond_cf(ymd("2021-02-01"), ymd("2030-02-01"), c(100.0), c(0.03), c(1L), ymd("2031-01-01"))
  expect_equal(nrow(out), 0L)

  out <- bond_result(ymd("2021-01-01", "2021-02-01"), ymd("2025-01-01", "2030-02-01"), c(100.0, NA), c(0.05, 0.03), c(0L, 1L), ymd("2022-01-01", "2022-02-01"), 100)
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

test_that("bond_result adjusts input to correct length and type", {
  out <- bond_result(211110, 20611110, 100, 0.04830, 2, 211130, c(109.83, 100))
  expect_equal(round(out[, "YTM"], 4), c(0.0436, 0.0489))
})

test_that("bond_result returns the same for leap and non-leap year", {
  out <- fcl::bond_result(
    value_date = c(200101, 210101),
    mty_date = c(210101, 220101),
    redem_value = 100,
    cpn_rate = 0.05,
    cpn_freq = 1,
    ref_date = c(200101, 210101),
    clean_price = 100
  )
  expect_equal(out$YTM, c(0.05, 0.05))
  expect_equal(out$MACD, c(1, 1))
  expect_equal(out$MODD, c(1 / 1.05, 1 / 1.05))
})
