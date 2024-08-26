#! /bin/bash

export host=Node2

docker run --privileged -it --rm \
-h $host \
--user $USER \
--name $host \
-v /home/$USER/workspace:/home/$USER/workspace \
network-tools:v1 bash