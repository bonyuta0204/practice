# Use Ubuntu as the base image
FROM ubuntu:latest


# Avoid prompts from apt
ENV DEBIAN_FRONTEND=noninteractive

# Update apt repository and install required packages
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    build-essential \
    curl \
    neovim \
    strace \
    git \
    gdb \
    gdbserver
#    && rm -rf /var/lib/apt/lists/* # Clean up to reduce image size

# Install Starship prompt
RUN curl -sS http://starship.rs/install.sh | sh

WORKDIR /app

# Set the default command to bash
CMD ["/bin/bash"]

