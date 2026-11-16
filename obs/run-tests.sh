#!/bin/sh
# Lance les tests de mode-console.lua sans OBS. L'interpréteur Lua n'est pas installé sur ce PC, mais la
# bibliothèque liblua5.4 l'est : on compile un lanceur minimal (lua54.c) la première fois.
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
RUNNER=${TMPDIR:-/tmp}/gl310-lua54
# La bibliothèque donnée par pkg-config si elle y est (NixOS…), sinon celle de Debian/Ubuntu.
LUA_LIBS=$(pkg-config --libs lua5.4 2>/dev/null || echo /usr/lib/x86_64-linux-gnu/liblua5.4.so.0)
# shellcheck disable=SC2086 # plusieurs options possibles
[ -x "$RUNNER" ] || cc -O2 -o "$RUNNER" "$HERE/lua54.c" $LUA_LIBS
cd "$HERE"
printf 'arg_script = "%s/mode-console.lua"\n' "$HERE" > "${TMPDIR:-/tmp}/gl310-lua-prelude.lua"
cat "${TMPDIR:-/tmp}/gl310-lua-prelude.lua" test_mode_console.lua > "${TMPDIR:-/tmp}/gl310-lua-test.lua"
"$RUNNER" "${TMPDIR:-/tmp}/gl310-lua-test.lua"
