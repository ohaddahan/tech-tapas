#!/bin/bash
set -x
source ./scripts/helpers.sh
function git_branch_name()
{
  echo "$(git symbolic-ref HEAD 2> /dev/null | awk 'BEGIN{FS="/"} {print $NF}')"
}

ensure export current_branch="$(git_branch_name)"
ensure git checkout -b "${current_branch}_backup"
ensure git push
ensure git checkout "${current_branch}"
