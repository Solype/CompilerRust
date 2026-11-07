#!/bin/bash
# Merges origin/main into every branch of origin and pushes it: no rebase, so no force push.
# A branch that already contains main is skipped. A merge with conflicts is aborted, listed,
# and the next branch goes on. The merges happen in a temporary worktree: the current checkout
# is not touched. A local branch of the same name is then fast-forwarded, unless it is checked
# out or has commits of its own (then: git pull on it).
# Usage: scripts/align_branches.sh [-h|--help] [-n|--dry-run] [branch...]
#   -n, --dry-run   list what would be merged; merge and push nothing
#   branch...       only these branches, named without origin/ (default: every branch but main)
# No environment variable. Example: scripts/align_branches.sh -n feat/6-pointers-and-arrays
# Exit code: 0 when every branch contains main, 1 on a conflict or a failed push, 2 on a wrong
# argument.

dry=0
branches=()
for arg in "$@"; do
    case "$arg" in
        -h | --help)
            sed -n '2,12p' "$0" | sed 's/^# \{0,1\}//'
            exit 0
            ;;
        -n | --dry-run) dry=1 ;;
        -*)
            echo "unknown option: $arg (see --help)" >&2
            exit 2
            ;;
        *) branches+=("$arg") ;;
    esac
done

cd "$(git rev-parse --show-toplevel)" || exit 2
git fetch -q --prune origin || exit 1
if [ ${#branches[@]} -eq 0 ]; then
    mapfile -t branches < <(git for-each-ref --format='%(refname:lstrip=3)' refs/remotes/origin |
        grep -vxE 'main|HEAD')
fi

WORK="$(mktemp -d)"
trap 'git worktree remove --force "$WORK/wt" 2> /dev/null; rm -rf "$WORK"' EXIT
git worktree add -q --detach "$WORK/wt" origin/main || exit 1

failed=0
update_local() { # update_local <branch>: fast-forward the local branch to origin, if there is one
    [ $dry -eq 0 ] && git show-ref -q --verify "refs/heads/$1" || return 0
    git fetch -q . "refs/remotes/origin/$1:refs/heads/$1" 2> /dev/null ||
        echo "      local $1 not updated: checked out or has its own commits, run git pull on it"
}

for b in "${branches[@]}"; do
    if ! git rev-parse -q --verify "refs/remotes/origin/$b" > /dev/null; then
        echo "FAIL  $b: no such branch on origin"
        failed=$((failed + 1))
        continue
    fi
    behind="$(git rev-list --count "origin/$b..origin/main")"
    if [ "$behind" -eq 0 ]; then
        echo "ok    $b"
        update_local "$b"
        continue
    fi
    if [ $dry -eq 1 ]; then
        echo "merge $b: $behind commits of main missing"
        continue
    fi
    git -C "$WORK/wt" checkout -q --detach "origin/$b"
    if ! git -C "$WORK/wt" merge -q --no-edit -m "Merge branch 'main' into $b" origin/main \
        > /dev/null 2>&1; then
        conflicts="$(git -C "$WORK/wt" diff --name-only --diff-filter=U | paste -sd ' ')"
        git -C "$WORK/wt" merge --abort
        echo "FAIL  $b: conflict in ${conflicts:-?}, merge it by hand"
        failed=$((failed + 1))
        continue
    fi
    if ! git -C "$WORK/wt" push -q origin "HEAD:refs/heads/$b"; then
        echo "FAIL  $b: push refused"
        failed=$((failed + 1))
        continue
    fi
    echo "done  $b: $behind commits of main merged"
    update_local "$b"
done

[ $failed -eq 0 ]
