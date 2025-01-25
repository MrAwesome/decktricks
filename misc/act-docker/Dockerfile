FROM archlinux:base-devel AS base
ENV CI=true
ENV GODOT4_BIN=/usr/bin/godot
RUN pacman -Syu --noconfirm git flatpak fontconfig p7zip rustup vim
RUN rustup default stable
#RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
#RUN ln -sf /root/.cargo/bin/cargo /usr/bin/cargo

# {{{
# NOTE: these are copied in like this instead of downloaded from elsewhere
#       because we need 4.4 to stop a terrible Linux bug in Godot,
#       and download times from tuxfamily are quite slow.
# TODO: Switch to fetching from github once 4.4 is officially released
RUN mkdir -p /root/.local/share/godot/export_templates/4.4.beta/
RUN mkdir -p /tmp/godot_extraction
COPY godot_binary.7z linux_release.7z /tmp/godot_extraction/
WORKDIR /tmp/godot_extraction
RUN 7z x godot_binary.7z 
RUN 7z x linux_release.7z
RUN cp godot /usr/bin/godot
WORKDIR /root/.local/share/godot/export_templates/4.4.beta/
RUN cp /tmp/godot_extraction/linux_release.x86_64 linux_release.x86_64
WORKDIR /
RUN rm -r /tmp/godot_extraction
# }}}

RUN mkdir -p /work
WORKDIR /work
#RUN apt-get -y install git 
# Seed cargo cache

# Install a tiny package (and all of flatpak's ridiculous dependencies)
# Otherwise, flatpak list will fail
RUN flatpak list

FROM base AS gitclone
RUN git clone https://github.com/MrAwesome/decktricks.git

FROM gitclone AS buildetc
#RUN apt-get -y install gcc pkg-config libssl-dev
WORKDIR /work/decktricks
RUN cargo build
RUN cargo build --release
RUN cargo test
RUN cargo test --release
WORKDIR /work/decktricks/gui/rust
# This handles actually building and placing the gui binary:
RUN cargo test --release
WORKDIR /work/decktricks

# Don't forget to run `docker container prune` followed by `docker image prune`
# to reclaim disk space if you've been testing here!
