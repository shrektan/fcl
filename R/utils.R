#' @importFrom ymd ymd
NULL

prepare_args <- function(...) {
  args <- list(...)
  lens <- vapply(args, length, integer(1))
  unique_len <- unique(lens)

  if (length(unique_len) == 1L) {
    return(args)
  }
  if (sum(unique_len != 1L) > 1L) {
    stop("all arguments must be the same length or length one")
  }

  n <- max(unique_len)
  rep_n <- function(x) {
    if (length(x) == n) {
      x
    } else if (length(x) == 1L) {
      rep(x, n)
    } else {
      stop("x's length must be 1 or ", n)
    }
  }
  args <- lapply(args, rep_n)
  args
}
