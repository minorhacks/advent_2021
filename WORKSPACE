workspace(name = "advent_2021")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

################################################################################
# Rust toolchain
################################################################################

http_archive(
    name = "rules_rust",
    sha256 = "531bdd470728b61ce41cf7604dc4f9a115983e455d46ac1d0c1632f613ab9fc3",
    strip_prefix = "rules_rust-d8238877c0e552639d3e057aadd6bfcf37592408",
    urls = [
        # `main` branch as of 2021-08-23
        "https://github.com/bazelbuild/rules_rust/archive/d8238877c0e552639d3e057aadd6bfcf37592408.tar.gz",
    ],
)

load("@rules_rust//rust:repositories.bzl", "rust_repositories")

rust_repositories()

################################################################################
# Cargo crates
################################################################################

load("//third_party/rules_rust:crate_universe_defaults.bzl", "DEFAULT_SHA256_CHECKSUMS", "DEFAULT_URL_TEMPLATE")
load("@rules_rust//crate_universe:defs.bzl", "crate_universe")

crate_universe(
    name = "crates",
    cargo_toml_files = [
        "//:Cargo.toml",
    ],
    resolver_download_url_template = DEFAULT_URL_TEMPLATE,
    resolver_sha256s = DEFAULT_SHA256_CHECKSUMS,
    supported_targets = [
        "x86_64-unknown-linux-gnu",
    ],
)

load("@crates//:defs.bzl", "pinned_rust_install")

pinned_rust_install()
