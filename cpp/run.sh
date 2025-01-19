#!/bin/bash

set -e

if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <code_path> [-f]"
  echo "  -f  Skip Test"
  exit 1
fi

if [[ ! -f "$1" ]] then
  echo "$1 does not exist."
  exit 1
fi

if [[ ! -f in.txt ]] then
  echo "in.txt does not exist."
  exit 1
fi

g++ -pass-exit-codes "$1"
if [[ $? -ne 0 ]] then
  exit 1
fi

if [[ ! $2 = "-f" ]] then
  if [[ ! -f test_in.txt ]] then
    echo "test_in.txt does not exist."
    exit 1
  fi
  if [[ ! -f test_out.txt ]] then
    echo "test_out.txt does not exist."
    exit 1
  fi
  ./a.out < test_in.txt | diff -y -Z test_out.txt -
  if [[ $? -ne 0 ]] then
    echo "test failed"
    exit 1
  fi

  echo "test passed! running with real input..."
fi

./a.out < in.txt | clip.exe
if [[ $? -ne 0 ]] then
  echo "an error occured while running."
  exit 1
fi

echo "succeeded! output copied to clipboard."
exit 0
