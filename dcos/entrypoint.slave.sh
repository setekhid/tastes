#!/usr/bin/dumb-init /bin/bash

dockerd \
	--storage-driver=vfs \
	--registry-mirror=https://ktm7bvr4.mirror.aliyuncs.com \
	--registry-mirror=https://registry.docker-cn.com &

sleep 15 && mesos-slave "$@"
