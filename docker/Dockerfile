FROM ubuntu

RUN apt-get update && apt-get install iproute2 -y --no-install-recommends && apt-get clean
COPY configure.sh /root/configure.sh
RUN chmod +x /root/configure.sh
CMD /root/configure.sh

#RUN tc qdisc add dev eth0 root netem loss random 5%
#CMD tail -f /dev/null