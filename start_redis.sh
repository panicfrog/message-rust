#!/bin/sh

# shellcheck disable=SC2164
CURRENT_FOLDER=$(cd "$(dirname "$0")";pwd)
docker run --name chat-redis \
  -v $CURRENT_FOLDER/redis/data:/data:rw \
  -v $CURRENT_FOLDER/redis/redis.conf:/etc/redis/redis.conf:ro \
  --privileged=true \
  -d redis redis-server /etc/redis/redis.conf