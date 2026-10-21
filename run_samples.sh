#!/bin/bash
# Compiles, links and runs each sample, then compares with what is expected:
# - sample/NN_*.kr: the exit code given by "`echo $?` après l'exécution affiche N"
#   in its header (0 without it), and the stdout in sample/NN_*.out if that file exists;
# - sample/erreurs/*.kr: the compiler must refuse it with code 1 (not a panic).
# Usage: ./run_samples.sh [file.kr...]   (all the samples without argument)

ROOT="$(cd "$(dirname "$0")" && pwd)"
COMPILER="$ROOT/target/debug/compiler"
cargo build -q --manifest-path "$ROOT/Cargo.toml" || exit 1

# a.out is written in the current directory: work in a temporary one
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT
cd "$WORK" || exit 1

if [ $# -eq 0 ]; then
    set -- "$ROOT"/sample/*.kr "$ROOT"/sample/erreurs/*.kr
fi

pass=0
fail=0
report() { # report <name> <message>, message empty when the sample passes
    if [ -z "$2" ]; then
        echo "PASS  $1"
        pass=$((pass + 1))
    else
        echo "FAIL  $1: $2"
        fail=$((fail + 1))
    fi
}
reason() { # the panic location and message, else the last line of stderr
    if grep -q 'panicked at' stderr; then
        grep -A 1 'panicked at' stderr | sed 's/^thread .* panicked at //' | tr '\n' ' '
    else
        tail -n 1 stderr
    fi
}

for kr in "$@"; do
    kr="$(realpath "$kr")"
    name="${kr#"$ROOT"/}"

    if [[ "$kr" == */erreurs/* ]]; then
        "$COMPILER" "$kr" > /dev/null 2> stderr
        code=$?
        if [ $code -eq 1 ]; then
            report "$name" ""
        else
            report "$name" "compiler exit $code, expected 1 ($(reason))"
        fi
        continue
    fi

    if ! "$COMPILER" -x "$kr" > /dev/null 2> stderr; then
        report "$name" "does not compile ($(reason))"
        continue
    fi
    expected="$(grep -oP 'après l.exécution affiche \K[0-9]+' "$kr" | head -n 1)"
    expected="${expected:-0}"
    # timeout: a wrong jump can loop forever
    timeout 5 ./a.out > stdout
    code=$?

    if [ $code -ne "$expected" ]; then
        report "$name" "exit $code, expected $expected"
    elif [ -f "${kr%.kr}.out" ] && ! diff -q "${kr%.kr}.out" stdout > /dev/null; then
        report "$name" "stdout differs from ${name%.kr}.out"
    else
        report "$name" ""
    fi
done

echo "$pass passed, $fail failed"
[ $fail -eq 0 ]
