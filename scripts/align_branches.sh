#!/bin/bash
# Rebases every branch of origin on origin/main and force-pushes it with a lease: the push is
# refused if someone pushed to the branch since the fetch. Merge commits are dropped, so the
# history of each branch becomes linear. A branch that already contains main is skipped.
# A rebase with conflicts is aborted, listed, and the next branch goes on. The rebases happen in
# a temporary worktree: the current checkout is not touched. A local branch of the same name is
# then moved to the rebased branch, unless it is checked out or has commits of its own (then:
# pull --rebase on it).
# Usage: scripts/align_branches.sh [-h|--help] [-n|--dry-run] [branch...]
#   -n, --dry-run   list what would be rebased; rebase and push nothing
#   branch...       only these branches, named without origin/ (default: every branch but main)
# No environment variable. Example: scripts/align_branches.sh -n feat/6-pointers-and-arrays
# Exit code: 0 when every branch contains main, 1 on a conflict or a failed push, 2 on a wrong
# argument.

dry=0
branches=()
for arg in "$@"; do
    case "$arg" in
        -h | --help)
            sed -n '2,14p' "$0" | sed 's/^# \{0,1\}//'
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
update_local() { # update_local <branch> <sha before the rebase>: move the local branch, if any
    [ $dry -eq 0 ] && git show-ref -q --verify "refs/heads/$1" || return 0
    local current
    current="$(git rev-parse "refs/heads/$1")"
    [ "$current" = "$(git rev-parse "refs/remotes/origin/$1")" ] && return 0
    if [ "$current" = "$2" ] && ! git worktree list --porcelain | grep -qx "branch refs/heads/$1"; then
        git update-ref "refs/heads/$1" "refs/remotes/origin/$1" "$2"
    else
        echo "      local $1 not updated: checked out or has its own commits, pull --rebase on it"
    fi
}

for b in "${branches[@]}"; do
    if ! old="$(git rev-parse -q --verify "refs/remotes/origin/$b")"; then
        echo "FAIL  $b: no such branch on origin"
        failed=$((failed + 1))
        continue
    fi
    behind="$(git rev-list --count "origin/$b..origin/main")"
    if [ "$behind" -eq 0 ]; then
        echo "ok    $b"
        update_local "$b" "$old"
        continue
    fi
    if [ $dry -eq 1 ]; then
        echo "rebase $b: $behind commits of main missing"
        continue
    fi
    git -C "$WORK/wt" checkout -q --detach "$old"
    if ! git -C "$WORK/wt" rebase -q origin/main > /dev/null 2>&1; then
        conflicts="$(git -C "$WORK/wt" diff --name-only --diff-filter=U | paste -sd ' ')"
        git -C "$WORK/wt" rebase --abort
        echo "FAIL  $b: conflict in ${conflicts:-?}, rebase it by hand"
        failed=$((failed + 1))
        continue
    fi
    if ! git -C "$WORK/wt" push -q --force-with-lease="refs/heads/$b:$old" origin "HEAD:refs/heads/$b"; then
        echo "FAIL  $b: push refused (pushed to since the fetch?)"
        failed=$((failed + 1))
        continue
    fi
    echo "done  $b: rebased on main ($behind commits of main)"
    update_local "$b" "$old"
done

[ $failed -eq 0 ]
