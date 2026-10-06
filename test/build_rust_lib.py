"""
Build the lib/rust workspace for the host so the gtest suites can link it.

Invoked by test/CMakeLists.txt when CROSSPOINT_RUST_LIB is ON. Builds the
workspace with the host toolchain into TARGET_DIR (so it never races the
firmware build in lib/rust/target) and copies each workspace crate's
cbindgen-generated header from its build script's OUT_DIR/rust into
INCLUDE_DIR/rust. A header is only rewritten when it changes, so an unchanged
crate does not force C++ recompiles. Headers in INCLUDE_DIR/rust that no crate
generated are removed.

Usage: build_rust_lib.py WORKSPACE_DIR TARGET_DIR PROFILE INCLUDE_DIR
"""

import json
import os
import subprocess
import sys


def main():
    workspace_dir, target_dir, profile, include_dir = sys.argv[1:]

    def cargo(args):
        cmd = [os.environ.get("CARGO", "cargo")] + args + ["--manifest-path", os.path.join(workspace_dir, "Cargo.toml")]
        env = dict(os.environ, CARGO_TARGET_DIR=target_dir)
        return subprocess.run(cmd, env=env, check=True, stdout=subprocess.PIPE, text=True).stdout

    metadata = json.loads(cargo(["metadata", "--no-deps", "--format-version=1"]))
    member_ids = set(metadata["workspace_members"])

    out_dirs = []
    # Diagnostics are rendered to stderr; stdout carries one JSON message per line.
    for line in cargo(["build", f"--profile={profile}", "--message-format=json-render-diagnostics"]).splitlines():
        msg = json.loads(line)
        # Cargo emits this message for fresh build scripts too, not only reruns.
        if msg.get("reason") == "build-script-executed" and msg.get("package_id") in member_ids:
            out_dirs.append(msg["out_dir"])
    if not out_dirs:
        sys.exit("build_rust_lib.py: cargo did not report any workspace build script OUT_DIR")

    # Headers live in a rust/ subdirectory so C++ includes them as <rust/name.h>.
    dest_dir = os.path.join(include_dir, "rust")
    os.makedirs(dest_dir, exist_ok=True)
    headers = set()
    for out_dir in out_dirs:
        src_dir = os.path.join(out_dir, "rust")
        for name in os.listdir(src_dir):
            if name.endswith(".h"):
                headers.add(name)
                copy_if_changed(os.path.join(src_dir, name), os.path.join(dest_dir, name))

    # Remove headers of renamed or removed crates so stale includes fail to build.
    for name in os.listdir(dest_dir):
        if name.endswith(".h") and name not in headers:
            os.remove(os.path.join(dest_dir, name))


def copy_if_changed(src, dest):
    with open(src, "rb") as f:
        header = f.read()
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
