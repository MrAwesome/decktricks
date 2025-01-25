#!/bin/bash

set -euxo pipefail

cd "$(dirname "$0")"

cp ~/godot/bin/godot.linuxbsd.editor.x86_64 ~/godot/bin/godot.linuxbsd.template_release.x86_64 .
mv godot.linuxbsd.editor.x86_64 godot
mv godot.linuxbsd.template_release.x86_64 linux_release.x86_64

rm linux_release.7z
rm godot_binary.7z

7z a linux_release.7z linux_release.x86_64
7z a godot_binary.7z godot

rm godot linux_release.x86_64
