test_that("rtn works", {

  out <- create_rtn(c(1, 1, 1), ymd(c(210101, 210105, 210110)), c(100, 103, 110), c(0, 3, 7))
  cr <- out$twrr_cr(1, ymd(210102), ymd(210110))
  dr <- out$twrr_dr(1, ymd(210102), ymd(210110))
  expect_equal(length(cr), 9L)
  expect_equal(length(dr), 9L)
  expect_equal(cr, cumprod(dr + 1) - 1)
  expect_equal(as.double(cr)[length(cr)], 0.1)

  dietz <- out$dietz(1, ymd(210102), ymd(210110))
  avc <- out$dietz_avc(1, ymd(210102), ymd(210110))
  expect_equal(as.double(dietz)[length(dietz)], 0.1)
  expect_equal(as.double(avc)[length(avc)], 100)
  expect_equal(length(dietz), 9L)
  expect_equal(length(avc), 9L)
})
