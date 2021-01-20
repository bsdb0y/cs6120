FROM ubuntu:latest
MAINTAINER neerajpal <neerajpal09@gmail.com>

ENV DEBIAN_FRONTEND=noninteractive
ENV APT_KEY_DONT_WARN_ON_DANGEROUS_USAGE=1

RUN apt-get update -y \
    && apt-get install nodejs -y \
    && apt-get install python3 -y \
    && apt-get install python3-pip -y \
    && apt-get install git -y \
    && apt-get install curl -y \
    && curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add - \
    && echo "deb https://dl.yarnpkg.com/debian/ stable main" >> /etc/apt/sources.list.d/yarn.list \
    && apt-get update -y \
    && apt-get install yarn -y

# Non-interactive modes get set back.
ENV DEBIAN_FRONTEND newt

RUN ln -s /usr/bin/python3.8 /usr/bin/python \
    && ln -s /usr/bin/pip3 /usr/bin/pip

WORKDIR /root

RUN git clone https://github.com/sampsyo/bril

# install typescript compiler for Bril
WORKDIR /root/bril/bril-ts

# as mentioned: https://capra.cs.cornell.edu/bril/tools/interp.html
RUN yarn \
    && yarn build \
    && yarn link

# install bril2json and bril2text
WORKDIR /root/bril/bril-txt

# to install as root user
ENV FLIT_ROOT_INSTALL=1

RUN pip install flit turnt \
    && flit install --symlink

ENV PATH=$PATH:'/root/.local/bin'

WORKDIR /mnt/host

# docker -t cs6120 .
# docker run -v `pwd`:/mnt/host --rm -it cs6120 bash
