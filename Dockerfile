FROM rust:alpine AS builder
COPY . /HPermission
WORKDIR /HPermission
RUN apk add musl-dev
RUN cargo build --release

FROM alpine
MAINTAINER longfangsong@icloud.com
COPY --from=builder /HPermission/target/release/hpermission /
WORKDIR /
CMD ["/hpermission"]