#!/usr/bin/env bash

BINARY_NAME="$*"

BINARY_PATH="./target/debug"

_usage(){
  echo "Usage: $0 <NAME_OF_RUNNING_BINARY>" 2>&1
  echo "binary must be located somewhere under ${BINARY_PATH}" 2>&1
  echo "instrument src/ code with '//BREAKPOINT' to be used as breakpoint by gdb"
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

echo "${BINARY_NAME} matches PID ${RUNNING_BINARY_PID}"

# TODO parse source-files, generate "break-points" at tagged locations in source-code file
# and feed these into the gdb command below
GDB_PARAMS=''
while IFS= read -r breakpoint; do
  if [ ! -z "${breakpoint}" ]; then
    GDB_PARAMS+="-ex 'break ${breakpoint}' "
  fi
done <<< "$(grep -ERn '(//|#) *BREAKPOINT' src/ | awk '{print $1}' | sed 's/.$//')"
GDB_PARAMS+="-ex 'continue'"

# attempt to attach debugger to the running process
CMD="sudo gdb "${GDB_PARAMS}" -q attach "${RUNNING_BINARY_PID}""
eval "set -x; ${CMD}"
