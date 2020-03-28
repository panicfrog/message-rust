#!/bin/sh

# shellcheck disable=SC2164
CURRENT_FOLDER=$(cd "$(dirname "$0")";pwd)
docker run --name chat-mysql \
  -v $CURRENT_FOLDER/mysql/data:/var/lib/mysql \
  -v $CURRENT_FOLDER/mysql/conf.d:/etc/mysql/conf.d \
  -e MYSQL_ROOT_PASSWORD=yeyongping \
  -d mysql:latest