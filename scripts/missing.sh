#!/bin/bash
# What is left to do, issue by issue: runs specs/<issue>-<slug>/ and lists what does not pass yet.
#
# Usage: scripts/missing.sh [-h|--help] [--all] [issue...]
#   (no argument)  on a branch <prefix>/<issue>-<slug>: that issue, passing checks included;
#                  on any other branch: every issue, only what is missing
#   --all          every issue, whatever the branch
#   issue...       these issues, passing checks included
# No environment variable. Exit code: 0 when nothing is missing, 1 otherwise, 2 on a wrong argument.
# Example: scripts/missing.sh 33 9
#
# A spec is one of:
#   specs/N-slug/*.kr         must compile and run: the rules of sample/*.kr (scripts/run_samples.sh)
#   specs/N-slug/errors/*.kr  must be refused with code 1: the rules of sample/errors/*.kr
#   specs/N-slug/checks.sh    check / manual lines, with the helpers below; its programs are in cases/
#   src/issue_specs.rs        `mod iN`: #[ignore] tests of what only the internals can show

if [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
    sed -n '2,16p' "$0" | sed 's/^# \{0,1\}//'
    exit 0
fi

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
# release: the debug compiler rebuilds the LR(1) table in 4 s for each program, this one in 0.5 s
export COMPILER="$ROOT/target/release/compiler"
JOBS=4

issues=()
mode=branch
for arg in "$@"; do
    case "$arg" in
        --all) mode=all ;;
        *[!0-9]*)
            echo "unknown argument: $arg (see --help)" >&2
            exit 2
            ;;
        *)
            issues+=("$arg")
            mode=some
            ;;
    esac
done
if [ "$mode" = branch ]; then
    current="$(git -C "$ROOT" branch --show-current)"
    if [[ "$current" =~ ^[a-z]+/([0-9]+)- ]] && compgen -G "$ROOT/specs/${BASH_REMATCH[1]}-*" > /dev/null; then
        issues=("${BASH_REMATCH[1]}")
        mode=some
    fi
fi

dirs=()
if [ "$mode" = some ]; then
    for n in "${issues[@]}"; do
        found=("$ROOT"/specs/"$n"-*/)
        if [ ! -d "${found[0]}" ]; then
            echo "no spec for issue $n" >&2
            exit 2
        fi
        dirs+=("${found[@]%/}")
    done
else
    mapfile -t dirs < <(ls -d "$ROOT"/specs/*/ | sed 's:/$::' | sort -V)
fi

cargo build -q --release --manifest-path "$ROOT/Cargo.toml" || exit 1
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT
RESULTS="$WORK/results"
: > "$RESULTS"

issue_of() { # issue_of <path under specs/>: the number of the issue
    local base="${1#"$ROOT"/specs/}"
    echo "${base%%-*}"
}
record() { # record <issue> <ok|MISS|ASK> <label> <reason>
    printf '%s\t%s\t%s\t%s\n' "$1" "$2" "$3" "$4" >> "$RESULTS"
}

# --- helpers of checks.sh: each prints why it fails and returns 1. Programs are relative to the
# spec, sources to the repository

check() { # check <label> <helper> <args...>: one line of the report
    local why
    if why="$("${@:2}" 2>&1)"; then
        record "$ISSUE" ok "$1" ""
    else
        record "$ISSUE" MISS "$1" "$(tail -n 1 <<< "$why")"
    fi
}
manual() { # manual <text>: a decision or a task no test can see, always listed
    record "$ISSUE" ASK "$1" ""
}
compile() { # sets $code and $err: the exit code and the stderr of the compiler on <file.kr>
    err="$("$COMPILER" "$1" 2>&1 > /dev/null)"
    code=$?
}
first_error() { # the panic location and message, else the last line of stderr
    if grep -q 'panicked at' <<< "$err"; then
        grep -A 1 'panicked at' <<< "$err" | sed 's/^thread .* panicked at //' | tr '\n' ' '
    else
        tail -n 1 <<< "$err"
    fi
}
compiled() { # compiled <file.kr>: compiles with code 0
    compile "$1"
    [ "$code" -eq 0 ] || { echo "compiler exit $code, expected 0 ($(first_error))"; return 1; }
}
rejects() { # rejects <file.kr> <regex>: refused with code 1, and the message matches
    compile "$1"
    [ "$code" -eq 1 ] || { echo "compiler exit $code, expected 1 ($(first_error))"; return 1; }
    grep -qE "$2" <<< "$err" || { echo "message without /$2/: $(first_error)"; return 1; }
}
complains() { # complains <file.kr> <regex>: an error or a warning matches, and no panic
    compile "$1"
    [ "$code" -le 1 ] || { echo "compiler exit $code ($(first_error))"; return 1; }
    grep -qE "$2" <<< "$err" || { echo "nothing matches /$2/ (exit $code): $(first_error)"; return 1; }
}
warns() { # warns <file.kr> <line>: compiles, with a warning on that line
    compiled "$1" || return 1
    grep -qE ":$2:[0-9]+: warning:" <<< "$err" || { echo "no warning on line $2"; return 1; }
}
silent() { # silent <file.kr>: compiles with nothing on stderr
    compiled "$1" || return 1
    [ -z "$err" ] || { echo "unexpected: $(first_error)"; return 1; }
}
no_panic() { # no_panic <file.kr>: compiles, or is refused with code 1
    compile "$1"
    [ "$code" -le 1 ] || { echo "compiler exit $code ($(first_error))"; return 1; }
}
source_has() { # source_has <regex> <path...>: the regex is in these files
    (cd "$ROOT" && grep -rqE "$1" "${@:2}") 2> /dev/null || { echo "/$1/ not in ${*:2}"; return 1; }
}
source_lacks() { # source_lacks <regex> <path...>: the regex is not in these files
    local hit
    hit="$(cd "$ROOT" && grep -rnE "$1" "${@:2}" 2> /dev/null | head -n 1)"
    [ -z "$hit" ] || { echo "still there: $hit"; return 1; }
}
file_exists() { # file_exists <glob>: a file of the repository matches
    (cd "$ROOT" && compgen -G "$1" > /dev/null) || { echo "no file $1"; return 1; }
}
disasm_has() { # disasm_has <file.kr> <regex>: the disassembly of its object file matches
    compiled "$1" || return 1
    objdump -d -M intel "${1%.kr}.o" | grep -qE "$2" || { echo "no /$2/ in the disassembly"; return 1; }
}
disasm_lacks() { # disasm_lacks <file.kr> <regex>: the disassembly of its object file does not match
    local hit
    compiled "$1" || return 1
    hit="$(objdump -d -M intel "${1%.kr}.o" | grep -E "$2" | head -n 1)"
    [ -z "$hit" ] || { echo "still in the disassembly: $hit"; return 1; }
}

# --- programs: run_samples.sh once per file, several at a time

programs=()
for dir in "${dirs[@]}"; do
    programs+=("$dir"/*.kr "$dir"/errors/*.kr)
done
for kr in "${programs[@]}"; do
    [ -e "$kr" ] && printf '%s\0' "$kr"
done | xargs -0 -r -n 1 -P "$JOBS" "$ROOT/scripts/run_samples.sh" 2> /dev/null > "$WORK/programs"
while IFS= read -r line; do
    case "$line" in
        "PASS  specs/"* | "FAIL  specs/"*) ;;
        *) continue ;;
    esac
    path="${line#????  }"
    path="${path%%: *}"
    label="${path#specs/*/}"
    label="${label%.kr}"
    if [ "${line:0:4}" = PASS ]; then
        record "$(issue_of "$ROOT/$path")" ok "$label" ""
    else
        reason="${line#*: }"
        record "$(issue_of "$ROOT/$path")" MISS "$label" "${reason//$ROOT\//}"
    fi
done < "$WORK/programs"

# --- checks.sh: one subshell per issue, several at a time

for dir in "${dirs[@]}"; do
    [ -f "$dir/checks.sh" ] || continue
    while [ "$(jobs -r | wc -l)" -ge "$JOBS" ]; do wait -n; done
    (
        ISSUE="$(issue_of "$dir")"
        cd "$dir" || exit 1
        # shellcheck disable=SC1091
        . ./checks.sh
    ) &
done
wait

# --- Rust specs: `mod iN` of src/issue_specs.rs, ignored by a plain `cargo test`

filters=()
for dir in "${dirs[@]}"; do
    n="$(issue_of "$dir")"
    if grep -q "^mod i$n {" "$ROOT/src/issue_specs.rs"; then
        filters+=("issue_specs::i$n::")
    fi
done
rust_broken=""
if [ ${#filters[@]} -gt 0 ]; then
    log="$WORK/rust.log"
    cargo test --manifest-path "$ROOT/Cargo.toml" --bin compiler -- --ignored "${filters[@]}" > "$log" 2>&1
    grep -q '^test result:' "$log" || rust_broken="$(grep -m 3 -E '^error' "$log")"
    # `test issue_specs::i4::name ... ok`, with ` - should panic` before the dots for some tests
    while read -r _ name rest; do
        n="${name#issue_specs::i}"
        n="${n%%::*}"
        if [ "${rest##* }" = ok ]; then
            record "$n" ok "${name##*::}" ""
        else
            # its panic message: left and right of an assert_eq, without the `panicked at` line
            why="$(grep -F -A 3 "'$name'" "$log" | grep -vE 'panicked at|^note:|^$|^--' | tr -s ' \n' ' ')"
            record "$n" MISS "${name##*::}" "$why"
        fi
    done < <(grep -E '^test issue_specs::i[0-9]+::' "$log")
fi

# --- report

verbose=$([ "$mode" = some ] && echo 1)
total_ok=0
total_miss=0
for dir in "${dirs[@]}"; do
    n="$(issue_of "$dir")"
    slug="$(basename "$dir")"
    awk -F'\t' -v n="$n" '$1 == n' "$RESULTS" | sort -t$'\t' -k3 > "$WORK/one"
    ok="$(awk -F'\t' '$2 == "ok"' "$WORK/one" | wc -l)"
    miss="$(awk -F'\t' '$2 == "MISS"' "$WORK/one" | wc -l)"
    total_ok=$((total_ok + ok))
    total_miss=$((total_miss + miss))
    if [ -n "$verbose" ] || [ "$miss" -gt 0 ] || grep -q $'\tASK\t' "$WORK/one"; then
        branch="$(git -C "$ROOT" for-each-ref --format='%(refname:lstrip=3)' "refs/remotes/origin/*/$slug" | head -n 1)"
        echo "#$n ${branch:-$slug}: $ok/$((ok + miss))"
        while IFS=$'\t' read -r _ status label reason; do
            case "$status" in
                ok) [ -z "$verbose" ] || echo "    ok    $label" ;;
                MISS) echo "    MISS  $label${reason:+: ${reason:0:200}}" ;;
                ASK) echo "    ASK   $label" ;;
            esac
        done < "$WORK/one"
    fi
done

echo
for dir in "${dirs[@]}"; do
    n="$(issue_of "$dir")"
    ok="$(awk -F'\t' -v n="$n" '$1 == n && $2 == "ok"' "$RESULTS" | wc -l)"
    miss="$(awk -F'\t' -v n="$n" '$1 == n && $2 == "MISS"' "$RESULTS" | wc -l)"
    state="$([ "$miss" -eq 0 ] && echo done || echo "$miss missing")"
    printf '#%-3s %3s/%-3s %-11s %s\n' "$n" "$ok" "$((ok + miss))" "$state" "$(basename "$dir")"
done
echo "$total_ok passed, $total_miss missing"
if [ -n "$rust_broken" ]; then
    echo "the Rust specs do not compile (cargo test --bin compiler): $rust_broken"
    exit 1
fi
[ "$total_miss" -eq 0 ]
