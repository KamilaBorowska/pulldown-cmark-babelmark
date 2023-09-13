# SPDX-FileCopyrightText: 2023 Konrad Borowski <konrad@borowski.pw>
#
# SPDX-License-Identifier: AGPL-3.0-or-later

{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell { buildInputs = [ cargo clippy openssl pkg-config rustfmt ]; }
