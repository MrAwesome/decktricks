#!/bin/bash

set -euxo pipefail

cd "$(dirname "$0")"

# For custom local builds:
# cp ~/godot/bin/godot.linuxbsd.editor.x86_64 ~/godot/bin/godot.linuxbsd.template_release.x86_64 ~/godot/bin/godot.linuxbsd.template_debug.x86_64 .
# mv godot.linuxbsd.editor.x86_64 godot
# mv godot.linuxbsd.template_release.x86_64 linux_release.x86_64
# mv godot.linuxbsd.template_debug.x86_64 linux_debug.x86_64

# TODO: brittle, breaks when system godot updates. fetch from upstream
cp /usr/bin/godot .
cp ~/.local/share/godot/export_templates/4.4.1.stable/linux_release.x86_64 .
cp ~/.local/share/godot/export_templates/4.4.1.stable/linux_debug.x86_64 .

rm -f linux_debug.7z linux_release.7z godot_binary.7z

7z a linux_debug.7z linux_debug.x86_64
7z a linux_release.7z linux_release.x86_64
7z a godot_binary.7z godot

rm godot linux_release.x86_64 linux_debug.x86_64
