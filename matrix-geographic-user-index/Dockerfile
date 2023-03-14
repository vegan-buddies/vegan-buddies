FROM debian:bookworm
RUN apt-get update && apt-get install -qqy openssl libgtk-3-0 libpq-dev libstdc++-10-dev cmake clang mold curl pkg-config libglib2.0-dev
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup ; sh rustup -y --no-modify-path ; chmod -R a+w $RUSTUP_HOME $CARGO_HOME;
RUN rustup toolchain install nightly
ARG UID
# https://stackoverflow.com/a/55757473
RUN useradd test --uid $UID
RUN mkdir /home/test ; chown test /home/test ; chgrp test /home/test
ENV CARGO_HOME=/home/test/.cargo
ENV PATH=$PATH:/home/test/.cargo/bin
