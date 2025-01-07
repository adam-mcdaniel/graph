#!/bin/bash

cargo doc --no-deps --all-features

mv target/doc docs