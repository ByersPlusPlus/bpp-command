# ByersPlusPlus Command Template

This is the scaffolding for every command library for ByersPlusPlus!

You can create your own command with this, thanks to `cargo-generate`.
Though you still have to implement command logic yourself, it takes care of bootstrapping the project for you.

## How to use this?

First and foremost, install `cargo-generate` by running `cargo install cargo-generate` if you haven't already. You only need to do this once on your machine.

Then you can run `cargo generate --git https://github.com/ByersPlusPlus/bpp-command.git`, which will ask you a few questions about the command, like the name of the project (which is also the name of the folder), the name of the command and the struct name of said command.

After filling these out, you can start implementing the logic for your command. You should be able to use pretty much any crate with this.

If you want to quickly add a new command, you can enter `bpp-cmd` in Visual Studio Code to generate another command structure, which you will only need to register down at the `register` function. Just copy and paste the generated line and replace the struct name. And of course fill in your own command details!

## How to compile this?

Compiling a command library itself is pretty straight forward.

If you compiled [commandservice](https://github.com/ByersPlusPlus/commandservice) on your machine without Docker, you can simply run `make`.

If you intend to use the Docker container for commandservice, please run `make docker-build`, else the commandservice might not be able to load your command library due to differing glibc versions.
