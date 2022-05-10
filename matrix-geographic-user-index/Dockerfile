FROM rust:buster
RUN apt-get update
RUN apt-get install -qqy openssl libgtk-3-0 libpq-dev

ARG UID
# https://stackoverflow.com/a/55757473
RUN useradd test --uid $UID
RUN mkdir /home/test ; chown test /home/test ; chgrp test /home/test
ENV CARGO_HOME=/home/test/.cargo
ENV PATH=$PATH:/home/test/.cargo/bin
