#!/bin/bash

docker build -t $1:http1 -f ../nginx-docker/Dockerfile.http1 ../nginx-docker/
docker build -t $1:http2 -f ../nginx-docker/Dockerfile.http2 ../nginx-docker/
docker build -t $1:http3 -f ../nginx-docker/Dockerfile.http3 ../nginx-docker/

docker build -t $1:http2-push -f ../nginx-docker/Dockerfile.http2push ../nginx-docker/

mkdir tmp

cp ../docker/Dockerfile tmp/Dockerfile
sed -i "s&SOURCEIMAGE&"$1":http1&g" tmp/Dockerfile
docker build -t $1-tc:http1 -f tmp/Dockerfile ../docker/

cp ../docker/Dockerfile tmp/Dockerfile
sed -i "s&SOURCEIMAGE&"$1":http2&g" tmp/Dockerfile
docker build -t $1-tc:http2 -f tmp/Dockerfile ../docker/

cp ../docker/Dockerfile tmp/Dockerfile
sed -i "s&SOURCEIMAGE&"$1":http3&g" tmp/Dockerfile
docker build -t $1-tc:http3 -f tmp/Dockerfile ../docker/

cp ../docker/Dockerfile tmp/Dockerfile
sed -i "s&SOURCEIMAGE&"$1":http2-push&g" tmp/Dockerfile
docker build -t $1-tc:http2push -f tmp/Dockerfile ../docker/

rm -r tmp
