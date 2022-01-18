#' @export
`[[.RRtn` <- `$.RRtn`

#' Create Rtn Object
#'
#' By providing a "group" (`ids`) of `dates`, `mvs` and `pls`,
#' calucating the time-weighted rate of rtn (twrr) or modified
#' dietz rate of return (dietz).
#'
#' @param date a Date vector, the reference date of each row
#' @param mv,pl a Double vector, the market value and the PnL of each day
#' @param id an integer vector, the ID of each row belongs to
#' @details All the input vector must be 1 or the same length.
#' @export
make_rtn <- function(date, mv, pl, id = 1L) {
  args <- prepare_args(
    ymd(date), as.double(mv), as.double(pl), as.integer(id)
  )
  obj <- do.call(RRtn$new, args)
  out <- new.env()
  out$.self <- obj
  id_default <- if (length(unique(id)) == 1L) {
    unique(id)
  }
  vars <- c("twrr_cr", "twrr_dr", "dietz", "dietz_avc", "cum_pl")
  lapply(vars, function(var) {
    fun <- function(from, to, id) {
      if (missing(id) && !is.null(id_default)) {
        id <- id_default
      }
      args <- prepare_args(
        from = ymd(from), to = ymd(to), id = as.integer(id), .len = 1L
      )
      .self <- out$.self
      with(args, xts::xts(
        .self[[var]](from, to, id),
        .self$dates(from, to)
      ))
    }
    assign(var, fun, envir = out)
  })
  out
}
