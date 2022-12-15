ARG SLE4_VERSION="2022_12_08"
FROM docker.io/trustworthysystems/sel4:${SLE4_VERSION}

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Create the user
RUN groupadd --gid $USER_GID $USERNAME && \
	useradd --uid $USER_UID --gid $USER_GID -m $USERNAME -s $(which bash) && \
	#
	# [Optional] Add sudo support. Omit if you don't need to install software after connecting.
	apt-get update && \
	apt-get install -y sudo && \
	echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME && \
	chmod 0440 /etc/sudoers.d/$USERNAME

USER ${USERNAME}

RUN sudo apt-get install -y clangd-13 && \
	sudo update-alternatives --install /usr/bin/clangd clangd /usr/bin/clangd-13 100 && \
	curl -sSL https://get.haskellstack.org/ | sh && \
	pip install aenum ordered_set plyplus pyelftools sortedcontainers

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN git clone https://github.com/corrosion-rs/corrosion.git /tmp/corrosion && \
	cmake -S /tmp/corrosion -B /tmp/corrosion/build -DCMAKE_BUILD_TYPE=Release && \
	cmake --build /tmp/corrosion/build --config Release && \
	sudo cmake --install /tmp/corrosion/build --config Release
