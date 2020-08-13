#!/bin/bash

printenv

sed -i "s/DOMAIN/"$DOMAINNAME"/g" ../conf/nginx.conf

cat ../conf/nginx.conf

./nginx -g 'daemon off;'