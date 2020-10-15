#!/bin/bash

/opt/google/chrome-unstable/chrome --remote-debugging-port=9222 --enable-quic --quic-version=h3-29 --origin-to-force-quic-on=<host>:<port> --enable-benchmarking --enable-net-benchmarking --disable-application-cache --media-cache-size=1 --disk-cache-size=1
