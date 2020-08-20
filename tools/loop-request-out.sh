#!/bin/bash
for i in `seq 1 $1`
do
	chrome-har-capturer -o $2.$i.har $3
done
