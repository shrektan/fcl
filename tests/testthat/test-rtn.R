test_that("rtn works", {
  out <- rtn(1, c(210101, 210105, 210110), c(100, 103, 110), c(0, 3, 7))
  cr <- out$twrr_cr(1, 210102, 210110)
  dr <- out$twrr_dr(1, 210102, 210110)
  expect_equal(length(cr), 9L)
  expect_equal(length(dr), 9L)
  expect_equal(cr, cumprod(dr + 1) - 1)
  expect_equal(as.double(cr)[length(cr)], 0.1)

  dietz <- out$dietz(1, 210102, 210110)
  avc <- out$dietz_avc(1, 210102, 210110)
  expect_equal(as.double(dietz)[length(dietz)], 0.1)
  expect_equal(as.double(avc)[length(avc)], 100)
  expect_equal(length(dietz), 9L)
  expect_equal(length(avc), 9L)
})

test_that("rtn will check input len", {
  expect_error(
    rtn(1:2, c(210101, 210105, 210110), c(100, 103, 110), c(0, 3, 7)),
    "length 1 or 3", fixed = TRUE
  )
  out <- rtn(1, c(210101, 210105, 210110), c(100, 103, 110), c(0, 3, 7))
  expect_error(
    out$twrr_cr(1:2, 210102, 210110),
    "must be length 1", fixed = TRUE
  )
})
