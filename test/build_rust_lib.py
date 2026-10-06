"""
Build lib/rust-lib for the host so the gtest suites can link it.

Invoked by test/CMakeLists.txt when CROSSPOINT_RUST_LIB is ON. Builds the
crate with the host toolchain into TARGET_DIR (so it never races the firmware
build in lib/rust-lib/target) and copies the cbindgen-generated rust_lib.h from
the build script's OUT_DIR into INCLUDE_DIR. The header is only rewritten when
it changes, so an unchanged crate does not force C++ recompiles.

Usage: build_rust_lib.py CRATE_DIR TARGET_DIR PROFILE INCLUDE_DIR
"""

import json
import os
import subprocess
import sys


def main():
    crate_dir, target_dir, profile, include_dir = sys.argv[1:]

    def cargo(args):
        cmd = [os.environ.get("CARGO", "cargo")] + args + ["--manifest-path", os.path.join(crate_dir, "Cargo.toml")]
        env = dict(os.environ, CARGO_TARGET_DIR=target_dir)
        return subprocess.run(cmd, env=env, check=True, stdout=subprocess.PIPE, text=True).stdout

    metadata = json.loads(cargo(["metadata", "--no-deps", "--format-version=1"]))
    package_id = metadata["packages"][0]["id"]

    out_dir = None
    # Diagnostics are rendered to stderr; stdout carries one JSON message per line.
    for line in cargo(["build", f"--profile={profile}", "--message-format=json-render-diagnostics"]).splitlines():
        msg = json.loads(line)
        # Cargo emits this message for fresh build scripts too, not only reruns.
        if msg.get("reason") == "build-script-executed" and msg.get("package_id") == package_id:
            out_dir = msg["out_dir"]
    if out_dir is None:
        sys.exit("build_rust_lib.py: cargo did not report the rust-lib build script's OUT_DIR")

    with open(os.path.join(out_dir, "rust_lib.h"), "rb") as f:
        header = f.read()
    dest = os.path.join(include_dir, "rust_lib.h")
    os.makedirs(include_dir, exist_ok=True)
    try:
        with open(dest, "rb") as f:
            if f.read() == header:
                return
    except FileNotFoundError:
        pass
    with open(dest, "wb") as f:
        f.write(header)


if __name__ == "__main__":
    main()
