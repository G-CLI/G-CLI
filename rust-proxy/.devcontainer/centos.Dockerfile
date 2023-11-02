FROM quay.io/centos/centos:centos7
RUN yum install -y gcc binutils
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install cargo-deb
RUN cargo install cargo-generate-rpm
RUN cargo install just
