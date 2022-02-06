##### DEV | CI | DEPLOY ENVIRONMENT #####
FROM centos:7 AS dev

RUN yum install -y centos-release-scl epel-release && yum install -y devtoolset-7-llvm llvm-toolset-7.0-lld && \
    # compile librocksdb-sys require gcc-c++ and libclang.so package from devtoolset-7-llvm
    echo "source scl_source enable devtoolset-7 llvm-toolset-7 llvm-toolset-7.0" >> /root/.bashrc && \
    # compile-time dynamic library requirement
    # - osshkeys/reqwest crate require openssl-devel
    # - vergen crate require libgit2.so (from git)
    # - jemalloc-sys crate require make
    yum install -y openssl-devel git make && \
    # locv(from epel-release registry) for cargo-tarpaulin genhtml report to gitlab
    yum install -y lcov && \
    yum clean all
# BASH_ENV make sure CI load .bashrc before run
ENV BASH_ENV "/root/.bashrc"

# Users in China can open this line to speed up the build.
# RUN git config --global url."https://github.com.cnpmjs.org/".insteadOf https://github.com/
SHELL ["/bin/bash", "--login", "-c"]

ENV NODEJS_LTS_VERSION v16.13.1
RUN curl -O -L https://npm.taobao.org/mirrors/node/latest-v16.x/node-$NODEJS_LTS_VERSION-linux-x64.tar.gz && \
    tar zxf node-$NODEJS_LTS_VERSION-linux-x64.tar.gz && \
    rm node-$NODEJS_LTS_VERSION-linux-x64.tar.gz
ENV PATH /node-$NODEJS_LTS_VERSION-linux-x64/bin/:$PATH


# Rustup
# Users in China can open this line to speed up the build.

# ENV RUSTUP_DIST_SERVER="https://rsproxy.cn" \
#     RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"

# RUN curl -sSf https://rsproxy.cn/rustup-init.sh | sh -s -- -y \
#     --no-modify-path \
#     --profile minimal \
#     --default-toolchain none

# COPY .cargo/docker_dev_cargo_config.toml /root/.cargo/config.toml

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

ENV PATH /root/.cargo/bin/:$PATH

# Prepare Rust toolchain from toolchain file
COPY rust-toolchain.toml .

# Install cargo tools
RUN mkdir cargo_install_target && \
    CARGO_TARGET_DIR=cargo_install_target cargo install cargo-chef cargo-tarpaulin cargo-udeps typos-cli && \
    rm -rf cargo_install_target



##### PLANNER with cargo-chef #####
FROM dev AS planner

# Users in China can open this line to speed up the build.
# COPY .cargo/docker_release_cargo_config.toml ~/.cargo/config.toml

WORKDIR /chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json



##### BUILD STAGE #####
FROM dev AS builder

WORKDIR /delicate

# Build deps
COPY --from=planner /chef/recipe.json recipe.json
RUN cargo chef cook --release --workspace --recipe-path recipe.json

# Build delicate
COPY . .
RUN cargo build --release --workspace && \
    mkdir /result && \
    cp target/release/delicate-actuator /result && \
    cp target/release/delicate-executor /result && \
    cp target/release/delicate-scheduler /result && \ 
    mv -r delicate-web/dist /result/delicate-web

# Build delicate-web
RUN npm install --legacy-peer-deps --prefix delicate-web/
RUN npm run build --prefix delicate-web/

##### FINAL RELEASE IMAGE #####
FROM centos:7 AS release

WORKDIR /delicate

ENV PATH=/delicate/bin:$PATH

ENV TZ=Etc/UTC \
    RUST_LOG=INFO

COPY --from=builder /result/delicate-actuator /delicate/bin/
COPY --from=builder /result/delicate-executor /delicate/bin/
COPY --from=builder /result/delicate-scheduler /delicate/bin/
COPY --from=builder /result/delicate-scheduler /delicate/bin/

EXPOSE 8090 8090
EXPOSE 9080 9080

VOLUME /delicate/data