#! /bin/bash

export host=Node1

docker run --privileged -it --rm \
-h $host \
--name $host \
-v /home/$USER/workspace:/home/$USER/workspace \
alpine:latest bash