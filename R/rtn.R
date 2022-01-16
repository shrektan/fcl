#' @export
`[[.RRtn` <- `$.RRtn`

#' Create Rtn Object
#'
#' By providing a "group" (`ids`) of `dates`, `mvs` and `pls`,
#' calucating the time-weighted rate of rtn (twrr) or modified
#' dietz rate of return (dietz).
#'
#' @param id an integer vector, the ID of each row belongs to
#' @param date a Date vector, the reference date of each row
#' @param mv,pl a Double vector, the market value and the PnL of each day
#' @details All the input vector must be 1 or the same length.
#' @export
create_rtn <- function(id, date, mv, pl) {
  args <- prepare_args(
    as.integer(id), ymd(date), as.double(mv), as.double(pl)
  )
  obj <- do.call(RRtn$new, args)
  out <- new.env()
  out$.self <- obj
  vars <- c("twrr_cr", "twrr_dr", "dietz", "dietz_avc", "cum_pl")
  lapply(vars, function(var) {
    fun <- function(id, from, to) {
      args <- prepare_args(
        id = as.integer(id), from = ymd(from), to = ymd(to), .len = 1L
      )
      .self <- out$.self
      with(args, xts::xts(
        .self[[var]](id, from, to),
        .self$dates(from, to)
      ))
    }
    assign(var, fun, envir = out)
  })
  out
}
