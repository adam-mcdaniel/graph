#!/bin/bash

cargo doc --no-deps --all-features

mv target/doc docs

# Add a redirect to the index.html file
echo "<meta http-equiv=refresh content=0;url=graph/index.html>" > docs/index.html