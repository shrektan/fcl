#' Create Rtn Object
#'
#' By providing a "group" (`ids`) of `dates`, `mvs` and `pls`,
#' calucating the time-weighted rate of rtn (twrr) or modified
#' dietz rate of return (dietz).
#'
#' @param ids an integer vector, the ID of each row belongs to
#' @param dates a Date vector, the reference date of each row
#' @param mvs,pls a Double vector, the market value and the PnL of each day
#' @details All the input vector must be the same length.
#' @export
create_rtn <- function(ids, dates, mvs, pls) {
  if (!inherits(dates, "Date")) {
    stop("dates must be Date")
  }
  if (is.integer(dates)) {
    dates <- structure(as.double(dates), class = "Date")
  }
  obj <- RRtn$new(as.integer(ids), dates, as.double(mvs), as.double(pls))
  n <- length(ids)
  if (length(dates) != n || length(mvs) != n || length(pls) != n) {
    stop("all the input must be the same length")
  }
  out <- new.env()
  out$.self <- obj
  out$twrr_cr <- function(id, from, to) {
    .self <- out$.self
    xts::xts(
      .self$twrr_cr(as.integer(id), from, to),
      .self$dates(from, to)
    )
  }
  out$twrr_dr <- function(id, from, to) {
    .self <- out$.self
    xts::xts(
      .self$twrr_dr(as.integer(id), from, to),
      .self$dates(from, to)
    )
  }
  out$dietz <- function(id, from, to) {
    .self <- out$.self
    xts::xts(
      .self$dietz(as.integer(id), from, to),
      .self$dates(from, to)
    )
  }
  out$dietz_avc <- function(id, from, to) {
    .self <- out$.self
    xts::xts(
      .self$dietz_avc(as.integer(id), from, to),
      .self$dates(from, to)
    )
  }
  out
}
