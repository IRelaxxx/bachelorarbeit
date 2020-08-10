#!/bin/bash

if [[ -z "${LOSS_PERCENT}" ]]; then
  LOSS_PERCENT="0%"
else
  LOSS_PERCENT="${LOSS_PERCENT}"
fi

tc qdisc add dev eth0 root netem loss random $LOSS_PERCENT