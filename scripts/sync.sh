#!/usr/bin/env bash
source ./scripts/helpers.sh
ensure git pull
ensure git submodule sync
ensure git submodule update --init --recursive
