"""
PlatformIO post-load script: after linking an Xtensa firmware, write
<progname>.noprop.elf, a copy of the ELF with the .xt.prop/.xt.lit property
tables removed.

objdump uses .xt.prop to decide which bytes are instructions and which are
literals. --gc-sections discards each object's main .xt.prop (it is tied to
the empty .text section under -ffunction-sections), so the linked table
covers only a small fraction of the code and objdump prints most functions
as raw words. Without the tables, objdump disassembles all code sections
linearly.
"""

import os

Import("env")  # noqa: F821  -- provided by PlatformIO at script load

cc = env.subst("$CC")  # noqa: F821
if os.path.basename(cc).startswith("xtensa-") and cc.endswith("-gcc"):
    # $OBJCOPY is esptool on this platform, so derive binutils objcopy from $CC.
    objcopy = cc[: -len("gcc")] + "objcopy"
    elf = "$BUILD_DIR/${PROGNAME}.elf"

    env.AddPostAction(  # noqa: F821
        elf,
        env.VerboseAction(  # noqa: F821
            f'"{objcopy}" -R .xt.prop -R .xt.lit $TARGET "$BUILD_DIR/${{PROGNAME}}.noprop.elf"',
            "Writing $BUILD_DIR/${PROGNAME}.noprop.elf",
        ),
    )
