#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(dirname "$(realpath "$0")")"

if [[ "${ROOT_DIR}" != "${PWD}" ]];
then
  echo "The script is supposed to be called from the repository root."
  exit 1
fi

source .session
HEADER="Cookie: session=${SESSION}"

DAY=1
YEAR=2021
INPUT_DIR="day/$(printf "%02d" $DAY)"
INPUT_FILE="${INPUT_DIR}/input"
INPUT_URL="https://adventofcode.com/${YEAR}/${DAY}/input"

mkdir -p "$INPUT_DIR"
echo "Created directory ${INPUT_DIR}"

curl --silent \
     -XGET \
     -H"${HEADER}" \
     --output "${INPUT_FILE}" \
     "${INPUT_URL}"
echo "Dowloaded input from ${INPUT_URL} to ${INPUT_FILE}"