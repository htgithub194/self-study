#! /bin/bash

netns_name=$1
mv_ip_address=$2

echo "Creating netns $netns_name ..."
sudo ip netns add $netns_name
sudo ip link add mv link eth0 type macvlan mode bridge
sudo ip link set mv netns $netns_name
sudo ip netns exec $netns_name ip link set dev mv up
sudo ip netns exec $netns_name ip a add $mv_ip_address dev mv
echo "Created netns $netns_name !"