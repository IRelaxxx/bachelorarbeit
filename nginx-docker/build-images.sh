#!/bin/bash

docker build -t $1:http1 -f Dockerfile.http1 .
docker build -t $1:http2 -f Dockerfile.http2 .
docker build -t $1:http3 -f Dockerfile.http3 .