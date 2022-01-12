#' @export
`[[.RRtn` <- `$.RRtn`

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
  vars <- c("twrr_cr", "twrr_dr", "dietz", "dietz_avc", "cum_pl")
  lapply(vars, function(var) {
    fun <- function(id, from, to) {
      .self <- out$.self
      xts::xts(
        .self[[var]](as.integer(id), from, to),
        .self$dates(from, to)
      )
    }
    assign(var, fun, envir = out)
  })
  out
}
