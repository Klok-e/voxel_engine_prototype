#/bin/bash

RUSTFLAGS='-C force-frame-pointers=y' cargo flamegraph --bin voxel_engine_prototype
