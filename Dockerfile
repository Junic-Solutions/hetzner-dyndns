FROM rust:1.75

WORKDIR /usr/src/hetzner-dyndns
COPY . .

RUN cargo install --path .

EXPOSE 3000
CMD ["hetzner-dyndns"]