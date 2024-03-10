FROM mcr.microsoft.com/vscode/devcontainers/rust:1-1-bullseye

# [Optional] Uncomment this section to install additional packages.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends libx11-dev libxtst-dev libudev-dev libinput-dev mingw-w64

RUN rustup target add x86_64-pc-windows-gnu && rustup toolchain install stable-x86_64-pc-windows-gnu