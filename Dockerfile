FROM rust:alpine
RUN apk add --no-cache musl-dev openssl-dev gtk4.0-dev libpq-dev

ARG UID
# https://stackoverflow.com/a/55757473
RUN adduser test --uid $UID --disabled-password --shell /bin/sh
RUN chown test /home/test ; chgrp test /home/test
