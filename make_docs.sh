#!/bin/bash

# A script to generate github-pages docs taken from 
# https://github.com/mitsuhiko/redis-rs/blob/master/upload-docs.sh

# Make a new repo for the gh-pages branch
rm -rf .gh-pages
mkdir .gh-pages
cd .gh-pages
git init

# Copy over the documentation
cp -r ../target/doc/* .
cat <<EOF > index.html
<!doctype html>
<title>redis</title>
<meta http-equiv="refresh" content="0; ./rust_dns_https/">
EOF

# Add, commit and push files
git add -f --all .
git commit -m "Built documentation"
git checkout -b gh-pages
git remote add origin https://github.com/gmosley/rust-DNSoverHTTPS.git
git push -qf origin gh-pages

# Cleanup
cd ..
rm -rf .gh-pages