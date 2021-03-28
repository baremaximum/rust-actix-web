#!/bin/bash
set -e

echo "Building function image..."

docker build -t rust-actix-web-template-test -f "../template/Dockerfile"

echo "test image build successful."