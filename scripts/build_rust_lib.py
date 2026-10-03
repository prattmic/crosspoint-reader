"""
PlatformIO pre-build script: build the lib/rust-lib crate as a static
library and link it into the firmware.

The crate path defaults to lib/rust-lib and can be overridden per env with
`custom_rust_lib_dir`. The Rust target is chosen from the board MCU; only the
ESP32-S3 (Xtensa) is wired up today. Defines CROSSPOINT_RUST_LIB=1 so sources
can guard calls into the library.
"""

import json
import os
import subprocess
import sys

Import("env")  # noqa: F821  (provided by SCons)

RUST_TARGETS = {
    "esp32s3": "xtensa-esp32s3-none-elf",
}
PROFILE = "release"


def fail(msg):
    print(f"ERROR [build_rust_lib.py]: {msg}", file=sys.stderr)
    env.Exit(1)  # noqa: F821


project_dir = env.subst("$PROJECT_DIR")  # noqa: F821
crate_dir = os.path.abspath(os.path.join(
    project_dir,
    env.GetProjectOption("custom_rust_lib_dir", "lib/rust-lib"),  # noqa: F821
))
mcu = env.BoardConfig().get("build.mcu")  # noqa: F821
target = RUST_TARGETS.get(mcu)
if target is None:
    fail(f"no Rust target configured for MCU '{mcu}'")

def run_cargo(args):
    cmd = ["cargo", "+esp"] + args
    print(f"build_rust_lib.py: {' '.join(cmd)} (in {crate_dir})")
    try:
        return subprocess.run(cmd, cwd=crate_dir, check=True, stdout=subprocess.PIPE, text=True).stdout
    except FileNotFoundError:
        fail("cargo not found on PATH; install rustup and the esp toolchain (espup)")
    except subprocess.CalledProcessError as e:
        fail(f"cargo {args[0]} failed (exit {e.returncode})")


def build_crate():
    """Build the crate and return the build script's OUT_DIR, which holds the
    generated rust_lib.h."""
    metadata = json.loads(run_cargo(["metadata", "--no-deps", "--format-version=1"]))
    package_id = metadata["packages"][0]["id"]

    # Build no_std (core only) for esp32. Diagnostics are rendered to stderr;
    # stdout carries one JSON message per line.
    out = run_cargo([
        "build", "-Zbuild-std=core", f"--profile={PROFILE}", f"--target={target}",
        "--message-format=json-render-diagnostics",
    ])
    for line in out.splitlines():
        msg = json.loads(line)
        # Cargo emits this message for fresh build scripts too, not only reruns.
        if msg.get("reason") == "build-script-executed" and msg.get("package_id") == package_id:
            return msg["out_dir"]
    fail("cargo did not report the rust-lib build script's OUT_DIR")


# Only build for actual firmware builds (not e.g. `pio run -t clean` / IDE
# metadata dumps, which still evaluate pre scripts).
if not (env.IsCleanTarget() or env.IsIntegrationDump()):  # noqa: F821
    env.Append(CPPPATH=[build_crate()])  # noqa: F821

env.Append(  # noqa: F821
    CPPDEFINES=[("CROSSPOINT_RUST_LIB", 1)],
    LIBPATH=[os.path.join(crate_dir, "target", target, PROFILE)],
    LIBS=["rust_lib"],
)
