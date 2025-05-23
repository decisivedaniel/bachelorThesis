FROM ubuntu:22.04

RUN apt update && apt install openssh-server sudo -y

RUN apt update && apt install build-essential llvm clang libclang-dev cmake libssl-dev pkg-config python3 git sudo -y

RUN apt install curl sudo -y

# RUN curl https://sh.rustup.rs -sSf 

RUN useradd -rm -d /home/ubuntu -s /bin/bash -g root -G sudo -u 1000 dev 

RUN  echo 'dev:dev' | chpasswd

RUN service ssh start

EXPOSE 22

CMD ["/usr/sbin/sshd","-D"]