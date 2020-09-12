FROM rust AS builder
COPY . /HPermission
WORKDIR /HPermission
RUN apt update && apt install -y cmake make gcc g++
RUN cargo build --release

FROM debian
MAINTAINER longfangsong@icloud.com
COPY --from=builder /HPermission/target/release/hpermission /
WORKDIR /
CMD ["/hpermission"]