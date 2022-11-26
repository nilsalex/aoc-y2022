#!/usr/bin/env bash

set -euo pipefail

YEAR=2021

ROOT_DIR="$(dirname "$(realpath "$0")")"

if [[ "${ROOT_DIR}" != "${PWD}" ]];
then
  echo "The script is supposed to be called from the repository root."
  exit 1
fi

if [[ ! -f .session ]]
then
  echo "File .session does not exist. Please provide it according to README.md."
  exit 1
fi
source .session
HEADER="Cookie: session=${SESSION}"

DAY="${1}"
PACKAGE_DIR="day$(printf "%02d" "${DAY}")"
INPUT_FILE="${PACKAGE_DIR}/input.txt"
INPUT_URL="https://adventofcode.com/${YEAR}/day/${DAY}/input"

if [[ -e ${PACKAGE_DIR} ]]
then
  echo "Package directory ${PACKAGE_DIR} already exists. Skipping package initialization."
else
  cargo new \
        --bin \
        --edition 2021 \
        --quiet \
        "${PACKAGE_DIR}"
  echo "Initialized cargo package at ${PACKAGE_DIR}"
fi

curl --silent \
     -XGET \
     -H"${HEADER}" \
     --output "${INPUT_FILE}" \
     "${INPUT_URL}"
echo "Downloaded input from ${INPUT_URL} to ${INPUT_FILE}. Character count: $(wc -c <"${INPUT_FILE}") "
