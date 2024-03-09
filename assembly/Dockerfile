# Use Alpine Linux for its small footprint
FROM alpine:latest

# Set the working directory in the container
WORKDIR /workspace

# Copy the current directory contents into the container at /usr/src/app
COPY . .

# Install build dependencies for C and Assembly
RUN apk update && apk add build-base bash nasm git neovim vim gdb curl fzf npm starship

RUN git clone https://github.com/bonyuta0204/dotfiles.git ~/dotfiles


RUN cd ~/dotfiles && ./setup -a


CMD ["/bin/bash"]
