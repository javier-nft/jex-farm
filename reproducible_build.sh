#!/bin/sh

ROOT_DIR=$(dirname $0)

if [ -d "${ROOT_DIR}/output-docker" ]
then
    read -p "Remove existing folder? (press Enter)" NULL

    rm -rf "${ROOT_DIR}/output-docker"
fi

mxpy contract reproducible-build --docker-image="multiversx/sdk-rust-contract-builder:v8.0.1"
