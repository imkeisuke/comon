FROM alpine:3.16 AS builder

ARG VERSION=0.1.0
ARG TARGETPLATFORM
ARG PLATFORM=${TARGETPLATFORM#linux/}

WORKDIR /home/comon

RUN apk add --no-cache curl tar gzip \
 && curl -LO https://github.com/imkeisuke/comon/releases/download/v${VERSION}/comon-${VERSION}_linux_${PLATFORM}.tar.gz \
 && tar xvfz totebag-${VERSION}_linux_${PLATFORM}.tar.gz 

FROM alpine:3.16

ARG VERSION=0.1.0

LABEL org.opencontainers.image.source https://github.com/imkeisuke/comon

RUN  apk add --no-cache libgcc musl-dev \
  && adduser -D nonroot \
  && mkdir -p /workdir

COPY --from=builder /home/comon/comon-${VERSION}/comon /opt/comon/comon

WORKDIR /workdir
USER nonroot

ENTRYPOINT [ "/opt/comon/comon" ]
