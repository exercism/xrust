#!/bin/bash
# This installs Rust 0.10 for Travis
set -x
set -e
yes | sudo add-apt-repository ppa:hansjorg/rust
sudo apt-get update
sudo apt-get -y install rust-0.10
