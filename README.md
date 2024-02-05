# jojo-common

This repository serves as a shared library among all components of the [jojo](https://github.com/gggiulio77/jojo). It contains all the business models and drivers adapters utilized throughout the project. 

## Getting Started

[Jojo](https://github.com/gggiulio77/jojo) comprises numerous independent components, making it susceptible to duplicated models across repositories. While striving for the [DRY](https://en.wikipedia.org/wiki/Don%27t_repeat_yourself) principle, there are challenges. For instance, this library is employed in software running on operating systems like Windows, but [jojo-client](https://github.com/gggiulio77/jojo-client) is an embedded application requiring compatibility with the [espressif xtensa](https://www.espressif.com/en/products/socs/esp32-s3) architecture.

To address this issue, feature flags have been introduced. The driver flag incorporates the implementation of keyboard, mouse, and gamepad drivers suitable for operating systems. Additionally, OS-specific flags such as windows, macos, and linux are utilized to select the appropriate keyboard layout, as each operating system has specific key configurations.

### Quick Links

- [Getting Started](#getting-started)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [Roadmap](#roadmap)
- [License](#license)

### Prerequisites

Before proceeding, ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system.

### Installation

`git clone https://github.com/gggiulio77/jojo-common.git`

## Usage

Just add jojo-common as a dependency in your `Cargo.toml`.

## Roadmap

- [ ] Implement cross platform for all drivers
- [ ] Improve error handling
- [ ] Improve CommandDiver adapter
- [ ] Update mouse/keyboard dependency

## License