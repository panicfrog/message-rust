#!/bin/sh

# shellcheck disable=SC2164
CURRENT_FOLDER=$(cd "$(dirname "$0")";pwd)
docker run --name chat-mysql \
  -p 3306:3306 \
  -v $CURRENT_FOLDER/mysql/data:/var/lib/mysql \
  -v $CURRENT_FOLDER/mysql/conf.d:/etc/mysql/conf.d \
  -v $CURRENT_FOLDER/mysql/init:/docker-entrypoint-initdb.d \
  -e MYSQL_RANDOM_ROOT_PASSWORD=yes \
  -d mysql:latest