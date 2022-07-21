#!/usr/bin/env bash

mkdir -p tmp

pushd tmp

svd2rust -i  ../DA14531.svd
form -i lib.rs -o src/
scp -r src ../

popd

rm -rf tmp

cargo fmt