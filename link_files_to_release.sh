#!/bin/bash

ln -s $(pwd)/assets/ ./target/release/assets
ln -s $(pwd)/config/ ./target/release/config

ln -s $(pwd)/assets/ ./target/debug/assets
ln -s $(pwd)/config/ ./target/debug/config
