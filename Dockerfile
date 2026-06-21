FROM rust:1.93.0-slim-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    libdbus-1-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY build ./build
COPY src/bin/envio/clap_app.rs ./src/bin/envio/clap_app.rs
RUN echo "fn main() {}" > src/bin/envio/main.rs && touch src/lib.rs \
    && cargo build --release --locked \
    && rm -rf src

COPY . .
RUN touch src/bin/envio/main.rs && cargo build --release --locked

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    gnupg \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -s /bin/bash envio

COPY --from=builder /app/target/release/envio /usr/local/bin/envio

RUN mkdir -p /app && chown -R envio:envio /app

RUN echo '' >> /home/envio/.bashrc \
    && echo 'echo -e "\033[0;32m✓ envio is installed and ready.\033[0m"' >> /home/envio/.bashrc \
    && echo 'echo "  Run '"'"'envio --help'"'"' to see available commands."' >> /home/envio/.bashrc \
    && chown envio:envio /home/envio/.bashrc


USER envio
WORKDIR /app
ENV SHELL=/bin/bash
ENV HOME=/home/envio

ENTRYPOINT ["/bin/bash"]