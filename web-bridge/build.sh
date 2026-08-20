#!/bin/sh

docker build -f Dockerfile-em -t a . && docker run -v ./web-bridge/dist:/dist -it a sh ./web-bridge/exp.sh