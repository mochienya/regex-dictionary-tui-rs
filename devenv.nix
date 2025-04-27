{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/basics/
  env.GREET = "devenv";

  # https://devenv.sh/packages/
  packages = [ pkgs.git ];

  # https://devenv.sh/languages/
  languages.rust = {
    enable = true;
    channel = "nightly";
    # targets = [ "x86_64-unknown-linux-gnu" "x86_64-pc-windows-gnu" ];
    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
  };

  # https://devenv.sh/scripts/
  scripts.hello.exec = ''
    echo hello from $GREET
  '';

  enterShell = ''
    git --version
  '';

  # https://devenv.sh/git-hooks/
  git-hooks.hooks = {
    clippy.enable = true;
    rustfmt.enable = true;
  };
}
