FROM fedora:latest@sha256:498c452f32a739b61f0ef215bce9924ebc4866cbe44710f58157d77723b7a6d2

# ---
# Setup base system ...
# ---

# Define the default user
ARG USERNAME=musicalninja
ARG USER_UID=1000
ARG USER_GID=${USER_UID}

# Enable man pages by commenting out the nodocs flag
COPY <<EOF /etc/dnf/dnf.conf
[main]
gpgcheck=True
installonly_limit=3
clean_requirements_on_remove=True
best=False
skip_if_unavailable=True
install_weak_deps=False
assumeyes=True
# tsflags=nodocs
EOF

# add default user & allow sudo
RUN groupadd --gid ${USER_GID} ${USERNAME} \
  && useradd --uid ${USER_UID} --gid ${USER_GID} -m ${USERNAME} \
  && echo ${USERNAME} ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/${USERNAME} \
  && chmod 0440 /etc/sudoers.d/${USERNAME}

# Install system packages ...
RUN dnf update \
  # man pages for all the stuff which is already installed
  && dnf reinstall --skip-unavailable $(dnf list --installed | awk '{print $1}') \
  # man itself, basic manpages, basic development tools
  && dnf install \
        bash-completion \
        git \
        man \
        man-db \
        man-pages \
        which


# ---
# Install rust ...
#   in /opt with a dedicated rust group
#   so we don't end up with system and user installs
#   this is a single user system.
# ---

ENV RUSTUP_HOME=/opt/rustup \
    CARGO_HOME=/opt/cargo \
    PATH=/opt/cargo/bin:$PATH

RUN \
# add foreign languages & linker used by rustc 
  dnf install \
      clang \
      mold \
      zig \
# add rust group
  && groupadd rust \
  && usermod -a -G rust root \
  && usermod -a -G rust ${USERNAME} \
# We run the whole rust install with user root:rust and umask 0002 but ...
  # Initial rust install directories still need to be created with correct mode
  && mkdir --mode=777 --parents $RUSTUP_HOME \
  && mkdir --mode=777 --parents $CARGO_HOME \
  # rustup always creates cargo/bin as root:root 755 unless we pre-create it here
  && mkdir --mode=775 --parents $CARGO_HOME/bin \
  && chgrp rust $CARGO_HOME/bin

USER root:rust
WORKDIR /opt
    # using ADD minimises layer sizes and improves caching as docker can check for changes
    ADD https://sh.rustup.rs rustup/rustup-init
    ADD --unpack=true https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz cargo/bin/
    # umask g+rwx
    RUN umask 0002 \
    && chmod a+x rustup/rustup-init \
    && rustup/rustup-init -v -y \
    # beta & nightly add 2Gb each to the image, but I use them a lot ...
    && rustup toolchain install stable beta nightly \
    && rustup component add \
            clippy \
            llvm-tools \
            llvm-tools-preview \
            rustfmt \
            rust-src \
    && cargo binstall --secure -y \ 
            cargo-about \
            cargo-cyclonedx \
            cargo-expand \
            cargo-machete \
            cargo-msrv \
            cargo-nextest \
            cargo-server \
            cargo-udeps \
            cargo-zigbuild \
            grcov \
            mdbook \
            ninja-xtask \
    # cargo-binstall creates binstall.toml as 600, ignoring umask
    && chmod 664 /opt/cargo/binstall.toml \
    # use mold linker by default
    && cat <<EOF >> ${CARGO_HOME}/config.toml
[target.'cfg(target_os = "linux")']
rustflags = ["-C", "link-arg=-fuse-ld=mold"]
EOF
USER root:root

# ---
# Final setup steps
# ---

# Set the default user
USER ${USERNAME}
