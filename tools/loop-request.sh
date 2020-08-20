#!/bin/bash
for i in `seq 1 $1`
do
	chrome-har-capturer $2
done
