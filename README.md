# nif_identicon

An Elixir/Rust NIF for generating (5x5) identicons. (very similar to identicons generated on Github)

## Examples

|<img src="./examples/elixir.png" width="100"> | <img src="./examples/github.png" width="100"> | <img src="./examples/helloworld.png" width="100"> | <img src="./examples/nif_identicon.png" width="100"> | <img src="./examples/rust.png" width="100"> 
|--- |--- |--- |--- |--- |
|<div align="center">`"elixir"`</div> | <div align="center">`"github"`</div> | <div align="center">`"helloworld"`</div> | <div align="center">`"nif_identicon"`</div> | <div align="center">`"rust"`</div> |

## Installation

⚠️ You must have the Rust toolchain installed in order to compile.

```elixir
defp deps do
  [{:nif_identicon, git: "https://github.com/asciialex/nif_identicon.git", tag: "0.1.0"}]
end
```

## Usage

Generate an identicon png binary or base64 string for any input string

```elixir
# Create some string for which you want to generate an identicon
input = "nif_identicon"

# For a binary of the png image
identicon_binary = NifIdenticon.generate_binary(input)

# For a base64 string of the png image
identicon_base64 = NifIdenticon.generate_base64(input)
```

