FROM rust:1.60.0

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

WORKDIR /app
COPY . .

RUN rustup default nightly
#RUN cargo update
RUN cargo build

CMD ["cargo", "run"]