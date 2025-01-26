#!/usr/bin/env bash

BINARY_NAME="$*"

BINARY_PATH="./target/debug"

_usage(){
  echo "Usage: $0 <NAME_OF_RUNNING_BINARY>" 2>&1
  echo "binary must be located somewhere under ${BINARY_PATH}"
  if [ "$#" -gt 0 ]; then
    echo "$*" 2>&1
  fi
  exit 1
}

# basic usage instructions
if [ -z "${BINARY_NAME}" ]; then
  _usage
fi

# get PID of binary if it's running, ignore the pid of the grep search itself
RUNNING_BINARY_PID="$(ps aux | grep "target/debug/.*${1}" | grep -v 'grep' | awk '{print $2}' | sed -n '1p')"

# empty result means process isn't running
if [ -z "${RUNNING_BINARY_PID}" ]; then
  _usage "No PID matching '${BINARY_NAME}', is the process running?"
fi

# attempt to attach debugger to the running process
sudo gdb attach "${RUNNING_BINARY_PID}"
