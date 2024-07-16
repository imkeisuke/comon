FROM rust:1-bullseye AS builder
ARG TARGETPLATFORM
WORKDIR /work/comon

COPY . .
RUN cargo build --release 

FROM debian:bullseye-slim
ARG VERSION=0.1.0

LABEL org.opencontainers.image.source https://github.com/imkeisuke/comon ¥
      org.opencontainers.image.version=${VERSION} ¥
      org.opencontainers.image.title=comon ¥
      org.opencontainers.image.description="comon is "
RUN  adduser --disabled-password --disabled-login --home /workdir nonroot ¥
     && mkdir -p /workdir
COPY --from=builder /work/comon/comon/release/comon /opt/comon/comon

WORKDIR /workdir
USER nonroot

ENTRYPOINT [ "/opt/comon/comon" ]
