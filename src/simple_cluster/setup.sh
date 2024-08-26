#! /bin/bash
dir=$(dirname $0); dir=$(cd $dir; pwd -P)
create_netns=$dir/create_netns.sh

$create_netns ns1 192.168.50.3/24
$create_netns ns2 192.168.50.4/24