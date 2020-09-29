#!/bin/bash

printenv

sed -i "s/DOMAIN/"$DOMAINNAME"/g" ../conf/nginx.conf

cat ../conf/nginx.conf

if test -f "/root/configure.sh"; then
    /root/configure.sh
fi

./nginx -g 'daemon off;'