# building step
FROM rust:1-bookworm AS build

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

# final step
FROM debian:bookworm-slim

WORKDIR /usr/src/app

EXPOSE 8080

# Install runtime dependencies
RUN apt-get update && \
  apt-get install -y libssl3 ca-certificates && \
  rm -rf /var/lib/apt/lists/*

COPY --from=build /usr/src/app/target/release/rinha-enferrujada-2025 /usr/src/app/

CMD ["/usr/src/app/rinha-enferrujada-2025"]
