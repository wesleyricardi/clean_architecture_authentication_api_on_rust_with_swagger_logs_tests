FROM rust:1.71

WORKDIR /app

RUN apt-get update
RUN rustup component add rustfmt
RUN rustup component add clippy
RUN cargo install sqlx-cli
RUN cargo install grcov
RUN rustup component add llvm-tools-preview
RUN apt-get install bc

CMD ["tail", "-f", "/dev/null"]