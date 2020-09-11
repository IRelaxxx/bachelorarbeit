#!/bin/bash

if [[ -n "${LOSS}" ]]; then
  tc qdisc add dev eth0 root netem loss random $LOSS
else
  echo "LOSS not set"
fi

if [[ -n "${RATE}" ]]; then
  if [[ -n "${BURST}" ]]; then
    if [[ -n "${LATENCY}" ]]; then
      tc qdisc add dev eth0 root tbf rate $RATE latency $LATENCY burst $BURST
    else
      echo "LATENCY not set"
    fi
  else
    echo "BURST not set"
  fi
else
  echo "RATE not set"
fi