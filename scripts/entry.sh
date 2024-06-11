#!/bin/bash -e
echo "Node version $(node -v)"
echo "NPM version $(npm -v)"

echo "ORG_URL: ${ORG_URL}"
echo "REG_TOKEN: ${REG_TOKEN}"
echo "RUNNER_NAME: ${RUNNER_NAME}"

./config.sh --unattended --url "${ORG_URL}" --token "${REG_TOKEN}" --name "${RUNNER_NAME}" --disableupdate
./run.sh
