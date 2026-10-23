# The following fields in DESCRIPTION can be used for customizing the behavior.
#
# Config/<package name>/MSRV (optional):
#   Minimum Supported Rust version (e.g. 1.41.0). If this is specified, errors
#   when the installed Rust compiler is older than the requirement.

SYSINFO_OS <- tolower(Sys.info()[["sysname"]])
SYSINFO_MACHINE <- Sys.info()[["machine"]]
HAS_32BIT_R <- dir.exists(file.path(R.home(), "bin", "i386"))
USE_UCRT <- identical(R.version$crt, "ucrt")


# Utilities ---------------------------------------------------------------

#' Read a field of the package's DESCRIPTION file
#'
#' The field should have the prefix
#'
#' @param field
#'   Name of a field without prefix (e.g. `"check_cargo"`).
#' @param prefix
#'   Prefix of the field (e.g. `"Config/rextendr/`).
#' @param optional
#'   If `TRUE`, return `NA` when there's no field. Otherwise raise an error.
#'
get_desc_field <- function(field, prefix = DESC_FIELD_PREFIX, optional = TRUE) {
  field <- paste0(prefix, field)
  if (length(field) != 1) {
    stop("Field must be length one of character vector")
  }

  # `read.dcf()` always succeeds even when the field is missing.
  # Detect the failure by checking NA
  x <- read.dcf("DESCRIPTION", fields = field)[[1]]

  if (isTRUE(is.na(x)) && !isTRUE(optional)) {
    stop("Failed to get the field ", field, " from DESCRIPTION")
  }

  x
}

# This is tricky; while DESC_FIELD_PREFIX is used in get_desc_field()'s default,
# this variable is defined by get_desc_field(). It's no problem as long as the
# default is not used before it exists!
DESC_FIELD_PREFIX <- paste0("Config/", get_desc_field("Package", prefix = ""), "/")


safe_system2 <- function(cmd, args) {
  result <- list(success = FALSE, output = "")

  output_tmp <- tempfile()
  on.exit(unlink(output_tmp, force = TRUE))

  suppressWarnings(ret <- system2(cmd, args, stdout = output_tmp))

  if (!identical(ret, 0L)) {
    return(result)
  }

  result$output <- readLines(output_tmp)
  result$success <- TRUE
  result
}

# check_rust_toolchain ----------------------------------------------------

#' Check if cargo and rustc exist and rustc meets the version requirement
#'
#' @return
#'   `TRUE` invisibly if no error was found.
check_rust_toolchain <- function() {
  ### Discover rustup's default binary dir before probing PATH ###
  # GUI / RStudio sessions often do not inherit ~/.cargo/bin from the shell.
  # Makevars still appends that directory later; the preflight must too.
  cargo_home <- Sys.getenv("CARGO_HOME", unset = file.path(path.expand("~"), ".cargo"))
  rust_bin <- file.path(cargo_home, "bin")
  if (dir.exists(rust_bin)) {
    Sys.setenv(PATH = paste(rust_bin, Sys.getenv("PATH"), sep = .Platform$path.sep))
  }

  ### Check if cargo and rustc work without error ###

  cat("*** Checking if cargo and rustc are installed\n")

  res_cargo <- safe_system2("cargo", "version")

  if (!isTRUE(res_cargo$success)) {
    stop(errorCondition("cargo command is not available", class = c("error_cargo_check", "error")))
  }

  res_rustc <- safe_system2("rustc", "--version")

  if (!isTRUE(res_rustc$success)) {
    stop(errorCondition("rustc command is not available", class = c("error_cargo_check", "error")))
  }

  ### Check the compiler version ###

  msrv <- get_desc_field("MSRV", optional = TRUE)

  if (isTRUE(!is.na(msrv))) {
    cat("*** Checking if rustc is new enough\n")

    version <- res_rustc$output

    ptn <- "rustc\\s+(\\d+\\.\\d+\\.\\d+)"
    m <- regmatches(version, regexec(ptn, version))[[1]]

    if (length(m) != 2) {
      stop(errorCondition("rustc version returned unexpected result", class = c("error_cargo_check", "error")))
    }

    if (package_version(m[2]) < package_version(msrv)) {
      msg <- sprintf("The installed version of rustc (%s) is older than the requirement (%s)", m[2], msrv)
      stop(errorCondition(msg, class = c("error_cargo_check", "error")))
    }
  }

  cat(sprintf("*** %s\n", res_cargo$output))
  cat(sprintf("*** %s\n", res_rustc$output))

  ### Check the targets ###

  if (identical(SYSINFO_OS, "windows")) {
    cat("*** Checking if the required Rust target is installed\n")

    targets <- safe_system2("rustup", c("target", "list", "--installed"))

    # rustup might not exist if Rust is installed directly via the .msi installer
    # in that case, just ignore and pray that the compilation will succeed.
    if (!isTRUE(targets$success)) {
      return(invisible(TRUE))
    }

    target <- if (grepl("aarch", R.version$platform)) {
      "aarch64-pc-windows-gnullvm"
    } else if (grepl("clang", Sys.getenv("R_COMPILED_BY"))) {
      "x86_64-pc-windows-gnullvm"
    } else if (grepl("i386", R.version$platform)) {
      "i686-pc-windows-gnu"
    } else {
      "x86_64-pc-windows-gnu"
    }
    if (!isTRUE(target %in% targets$output)) {
      msg <- paste("The required target", target, "is not installed")
      stop(errorCondition(msg, class = c("error_cargo_check", "error")))
    }
  }

  invisible(TRUE)
}

# MAIN --------------------------------------------------------------------

### Check cargo toolchain ###

cargo_check_result <- tryCatch(
  check_rust_toolchain(),
  # Defer errors if it's raised by functions here
  error_cargo_check = function(e) e$message
)

# This package has no pre-compiled binary fallback. A missing or too-old
# toolchain must fail configure so installation does not proceed offline
# without cargo/rustc.
if (isTRUE(cargo_check_result)) {
  cat("*** Rust toolchain is ok\n")
  quit("no", status = 0)
}

cat(sprintf("
-------------- ERROR: CONFIGURATION FAILED --------------------

[Rust toolchain check result]
%s

Please refer to <https://www.rust-lang.org/tools/install> to install Rust.

---------------------------------------------------------------

", cargo_check_result))
quit("no", status = 2)
