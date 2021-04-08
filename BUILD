load("@rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "proxy_wasm_filter_echo",
    srcs = glob(["src/*.rs"]),
    crate_type = "cdylib",
    out_binary = True,
    edition = "2018",
    visibility = ["//visibility:public"],
    proc_macro_deps = [
        "//bazel/cargo:serde_derive",
    ],
    deps = [
        "//bazel/cargo:proxy_wasm",
        "//bazel/cargo:chrono",
        "//bazel/cargo:http",
        "//bazel/cargo:log",
        "//bazel/cargo:serde",
        "//bazel/cargo:serde_json",
        "//bazel/cargo:serde_tuple_vec_map",
    ],
)
