#!/bin/bash

# This script is meant for CI only.

set -ex

wget https://github.com/mozilla/geckodriver/releases/download/v0.23.0/geckodriver-v0.23.0-linux64.tar.gz
tar xzf geckodriver-v0.23.0-linux64.tar.gz
