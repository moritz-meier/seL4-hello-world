cmake_minimum_required(VERSION 3.18.4)
project(rust-hello-world C)

add_subdirectory(../../corrosion corrsion_bin)

corrosion_import_crate(MANIFEST_PATH Cargo.toml NO_STD)
