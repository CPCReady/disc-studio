# MIT License - Copyright (c) 2026 Destroyer
# Homebrew formula for xdsk — Modern DSK image manipulation tool for Amstrad CPC
#
# To use this formula before it is merged into homebrew-core, create a tap:
#   brew tap GITHUB_OWNER/xdsk https://github.com/GITHUB_OWNER/xdsk
#   brew install GITHUB_OWNER/xdsk/xdsk
#
# Replace GITHUB_OWNER with the actual GitHub username/organization.
# The sha256 checksum must be updated on every release.

class Xdsk < Formula
  desc "xdsk - Modern DSK image manipulation tool for Amstrad CPC"
  homepage "https://github.com/GITHUB_OWNER/xdsk"
  url "https://github.com/GITHUB_OWNER/xdsk/archive/refs/tags/v1.0.0.tar.gz"
  sha256 "REPLACE_WITH_SHA256_OF_SOURCE_TARBALL"
  license "MIT"
  head "https://github.com/GITHUB_OWNER/xdsk.git", branch: "main"

  # Stable bottle hashes are filled in automatically by the Homebrew bot
  # after the formula is merged into homebrew-core.
  # bottle do
  #   ...
  # end

  depends_on "rust" => :build

  def install
    # The Rust project lives in the disc/ subdirectory
    system "cargo", "install", *std_cargo_args(path: "disc")

    # Install shell completions
    generate_completions_from_executable(bin/"xdsk", "completions")

    # Install man pages
    man1.mkpath
    system bin/"xdsk", "--no-color", "mangen", man1
  end

  test do
    # Verify the binary runs
    assert_match "xdsk #{version}", shell_output("#{bin}/xdsk --version")

    # Create a DSK image and list its (empty) contents
    system bin/"xdsk", "--no-color", "create", testpath/"test.dsk"
    output = shell_output("#{bin}/xdsk --no-color list #{testpath}/test.dsk")
    assert_match "0 files", output
  end
end
