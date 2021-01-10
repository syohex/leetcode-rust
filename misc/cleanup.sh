#!/usr/bin/env bash
set -e
set -x

find . -type d -name 'target' | xargs rm -rf
