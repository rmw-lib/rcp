#!/usr/bin/env bash

DIR=$(dirname $(realpath "$0"))
cd $DIR
set -ex
. ./sh/pid.sh
exec cargo run -p rmw
