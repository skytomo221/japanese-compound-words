FROM debian:latest AS corpus

RUN apt-get update && apt-get install -y curl

ARG jawiki_revision="latest"
RUN curl https://dumps.wikimedia.org/jawiki/${jawiki_revision}/jawiki-${jawiki_revision}-pages-articles.xml.bz2 -o jawiki.xml.bz2

WORKDIR /workspace
CMD [ "mv", "/jawiki.xml.bz2", "." ]

FROM python:3.10 AS wikiextractor

WORKDIR /workspace

RUN pip install wikiextractor
CMD [ "python", "-m", "wikiextractor.WikiExtractor", "-b", "10G", "-o", "corpus", "jawiki.xml.bz2" ]

FROM rust:latest AS generator

RUN apt-get update && apt-get install -y \
    mecab \
    libmecab-dev \
    mecab-ipadic-utf8 \
    git \
    make \
    curl \
    xz-utils \
    file \
    wget \
    patch \
    bash \
    swig \
    gfortran \
    g++ \
    sudo

RUN git clone --depth 1 https://github.com/neologd/mecab-ipadic-neologd.git && \
    cd mecab-ipadic-neologd && \
    bin/install-mecab-ipadic-neologd -n -y && \
    cd .. && \
    rm -r mecab-ipadic-neologd

CMD [ "cargo", "run" ]
