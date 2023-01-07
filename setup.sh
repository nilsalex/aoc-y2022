#!/usr/bin/env bash

set -euo pipefail

YEAR=2022

if [[ -z "${1+x}" ]]
then
  echo "Please provide the day as argument."
  exit 1
fi

if [[ ! "${1}" =~ ^([1-9]|1[0-9]|2[0-5])$ ]];
then
  echo "Argument must be a number from 1 to 25."
  exit 1
fi

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
INPUT_FILE="$(printf "inputs/day%02d.txt" "${DAY}")"
INPUT_URL="https://adventofcode.com/${YEAR}/day/${DAY}/input"

curl --silent \
     -XGET \
     -H"${HEADER}" \
     --output "${INPUT_FILE}" \
     "${INPUT_URL}"
echo "Downloaded input from ${INPUT_URL} to ${INPUT_FILE}. Character count: $(wc -c <"${INPUT_FILE}") "
