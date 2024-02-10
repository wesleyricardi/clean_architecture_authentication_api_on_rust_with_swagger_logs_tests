#!/bin/bash

COVERAGE_FILE="coverage/tests.lcov"
DESIRED_COVERAGE=75

if [ ! -f "$COVERAGE_FILE" ]; then
  echo "Coverage file '$COVERAGE_FILE' was not found."
  exit 1
fi


LF=$(grep -oP 'LF:(\d+)' "$COVERAGE_FILE" | awk -F ":" '{sum += $2} END {print sum}')
LH=$(grep -oP 'LH:(\d+)' "$COVERAGE_FILE" | awk -F ":" '{sum += $2} END {print sum}')


if [ "$LF" -eq 0 ]; then
  COVERAGE=0
else
  COVERAGE=$(echo "scale=2; ($LH / $LF) * 100" | bc)
fi


COVERAGE_INT=$(echo "$COVERAGE" | cut -d'.' -f1)

if [ "$COVERAGE_INT" -ge "$DESIRED_COVERAGE" ]; then
  echo "Code coverage meets $DESIRED_COVERAGE% criterion."
else
  echo "Code coverage is below $DESIRED_COVERAGE% criterion."
  exit 1
fi
