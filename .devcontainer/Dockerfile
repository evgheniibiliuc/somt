FROM mcr.microsoft.com/devcontainers/base:ubuntu
# Install the xz-utils package
RUN apt-get update && apt-get install -y xz-utils
RUN apt-get install cargo -y
RUN apt-get install curl -y
RUN curl --proto '=https' --tlsv1.2 -sSf -y https://sh.rustup.rs  | sh

RUN git config --global user.email "evgheniyhome@gmail.com"
RUN git config --global user.email "evgheniyhome@gmail.com" && git config --global user.name "evgheniibiliuc"
