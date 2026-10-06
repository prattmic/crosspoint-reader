"""
PlatformIO pre-build script: build the lib/rust Cargo workspace and link its
librust_lib.a (the workspace's staticlib crate) into the firmware.

The workspace path defaults to lib/rust and can be overridden per env with
`custom_rust_lib_dir`. Each workspace crate generates its own C++ header (e.g.
rust_epub.h); every one is added to the include path. The Rust target is chosen
from the board MCU; only the ESP32-S3 (Xtensa) is wired up today. The crates
build against std using the ESP-IDF targets; std's libc/pthread dependencies
resolve against the ESP-IDF libraries that Arduino-ESP32 already links. Defines
CROSSPOINT_RUST_LIB=1 so sources can guard calls into the library.
"""

import json
import os
import subprocess
import sys

Import("env")  # noqa: F821  (provided by SCons)

RUST_TARGETS = {
    "esp32s3": "xtensa-esp32s3-espidf",
}
PROFILE = "release"


def fail(msg):
    print(f"ERROR [build_rust_lib.py]: {msg}", file=sys.stderr)
    env.Exit(1)  # noqa: F821


project_dir = env.subst("$PROJECT_DIR")  # noqa: F821
workspace_dir = os.path.abspath(os.path.join(
    project_dir,
    env.GetProjectOption("custom_rust_lib_dir", "lib/rust"),  # noqa: F821
))
mcu = env.BoardConfig().get("build.mcu")  # noqa: F821
target = RUST_TARGETS.get(mcu)
if target is None:
    fail(f"no Rust target configured for MCU '{mcu}'")

def cargo_env():
    """Environment for cargo. ESP-IDF >= 5 uses a 64-bit time_t, which the
    libc crate (and thus std) only matches with --cfg espidf_time64."""
    cargo_env = dict(os.environ)
    var = "CARGO_TARGET_" + target.upper().replace("-", "_") + "_RUSTFLAGS"
    cargo_env[var] = " ".join(filter(None, [cargo_env.get(var), "--cfg espidf_time64"]))
    return cargo_env


def run_cargo(args):
    cmd = ["cargo", "+esp"] + args
    print(f"build_rust_lib.py: {' '.join(cmd)} (in {workspace_dir})")
    try:
        return subprocess.run(cmd, cwd=workspace_dir, env=cargo_env(), check=True, stdout=subprocess.PIPE,
                              text=True).stdout
    except FileNotFoundError:
        fail("cargo not found on PATH; install rustup and the esp toolchain (espup)")
    except subprocess.CalledProcessError as e:
        fail(f"cargo {args[0]} failed (exit {e.returncode})")


def build_workspace():
    """Build the workspace and return the OUT_DIR of each workspace crate's
    build script, which holds that crate's generated header."""
    metadata = json.loads(run_cargo(["metadata", "--no-deps", "--format-version=1"]))
    member_ids = set(metadata["workspace_members"])

    # The esp toolchain ships no prebuilt std for the ESP-IDF targets, so build
    # it from source. Diagnostics are rendered to stderr; stdout carries one
    # JSON message per line.
    out = run_cargo([
        "build", "-Zbuild-std=std,panic_abort", f"--profile={PROFILE}", f"--target={target}",
        "--message-format=json-render-diagnostics",
    ])
    out_dirs = []
    for line in out.splitlines():
        msg = json.loads(line)
        # Cargo emits this message for fresh build scripts too, not only reruns.
        if msg.get("reason") == "build-script-executed" and msg.get("package_id") in member_ids:
            out_dirs.append(msg["out_dir"])
    if not out_dirs:
        fail("cargo did not report any workspace build script OUT_DIR")
    return out_dirs


# Only build for actual firmware builds (not e.g. `pio run -t clean` / IDE
# metadata dumps, which still evaluate pre scripts).
if not (env.IsCleanTarget() or env.IsIntegrationDump()):  # noqa: F821
    env.Append(CPPPATH=build_workspace())  # noqa: F821

env.Append(  # noqa: F821
    CPPDEFINES=[("CROSSPOINT_RUST_LIB", 1)],
    LIBPATH=[os.path.join(workspace_dir, "target", target, PROFILE)],
    LIBS=["rust_lib"],
)
