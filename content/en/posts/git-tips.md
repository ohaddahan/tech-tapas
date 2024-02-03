+++
title = 'Tips using Git'
date = 2024-01-28T09:22:33+02:00
draft = false
+++

## Tip #1: git hooks

[githooks](https://git-scm.com/docs/githooks) are scripts that run `pre`/`post` every `git` command you run in your
repository.

Using [githooks](https://git-scm.com/docs/githooks) you can ensure all team members run certain checks prior to doing
various actions, such as `commit`.

To create a [githooks](https://git-scm.com/docs/githooks) check `.git/hooks` directory in your repository.
It contains sample scripts, to enable one, rename the script and remove the `.sample` suffix.

### Pre-commit

### Pre-rebase

## Tip #2:

`alias gitfsmonitor_status='git fsmonitor--daemon status'`
