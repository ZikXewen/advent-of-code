#!/bin/bash

set -e

if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <code_path> [flags]"
  echo "  -f  Skip test"
  echo "  -s  Print output to stdout instead of clipboard"
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

if [[ ! ($2 = "-f" || $3 = "-f") ]] then
  if [[ ! -f test_in.txt ]] then
    echo "test_in.txt does not exist."
    exit 1
  fi
  if [[ ! -f test_out.txt ]] then
    echo "test_out.txt does not exist."
    exit 1
  fi
  python3 "$1" < test_in.txt | diff -y -Z test_out.txt -
  if [[ $? -ne 0 ]] then
    echo "test failed"
    exit 1
  fi

  echo "test passed! running with real input..."
fi

if [[ $2 = "-s" || $3 = "-s" ]] then
  python3 "$1" < in.txt
else
  python3 "$1" < in.txt | clip.exe
fi

if [[ $? -ne 0 ]] then
  echo "an error occured while running."
  exit 1
fi

echo "succeeded! output copied to clipboard."
exit 0
