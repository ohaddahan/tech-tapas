+++
title = 'Tips using Git'
date = 2024-01-28T09:22:33+02:00
draft = false
+++

## Tip #1: git hooks

[git hooks](https://git-scm.com/docs/githooks) are scripts that run `pre`/`post` every `git` command you run in your
repository.

Using [git hooks](https://git-scm.com/docs/githooks) you can ensure all team members run certain checks prior to doing
various actions, such as `commit`.

To create a [git hooks](https://git-scm.com/docs/githooks) check `.git/hooks` directory in your repository.
It contains sample scripts, to enable one, rename the script and remove the `.sample` suffix.

### Pre-commit

One of the most common hooks is `pre-commit`, usually used for running formatters to avoid "formatting wars" between
users.

For example [prettier](https://prettier.io/docs/en/install) instructions how to enable `prettier` as a `pre-commit`
hook.
This is effectively a must-have for any project with more than one developer.

### Pre-rebase

`rebase` is a powerful tool, but it can be dangerous and lead to lose of work by accident.
One way to prevent this is to backup your current branch using a `pre-rebase` hook.

This `hook` simple creates a new `branch` and `push` it, and returns to the original `branch`.

Simple yet highly effective.

## Tip #2:

[fsmonitor--daemon](https://git-scm.com/docs/git-fsmonitor--daemon) improves performance of `git` commands by listening
to file system changes and updating the index accordingly.

This is very significant for large repositories and have little down side to using it.

## References

* [git hooks](https://git-scm.com/docs/githooks)
* [Improve Git monorepo performance with a file system monitor](https://github.blog/2022-06-29-improve-git-monorepo-performance-with-a-file-system-monitor/)