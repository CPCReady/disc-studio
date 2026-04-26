# MIT License - Copyright (c) 2026 Destroyer
# Homebrew formula for disc — Modern DSK image manipulation tool for Amstrad CPC
#
# To use this formula before it is merged into homebrew-core, create a tap:
#   brew tap GITHUB_OWNER/disc https://github.com/GITHUB_OWNER/disc-image-studio
#   brew install GITHUB_OWNER/disc/disc
#
# Replace GITHUB_OWNER with the actual GitHub username/organization.
# The sha256 checksum must be updated on every release.

class Disc < Formula
  desc "Modern DSK image manipulation tool for Amstrad CPC"
  homepage "https://github.com/GITHUB_OWNER/disc-image-studio"
  url "https://github.com/GITHUB_OWNER/disc-image-studio/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "REPLACE_WITH_SHA256_OF_SOURCE_TARBALL"
  license "MIT"
  head "https://github.com/GITHUB_OWNER/disc-image-studio.git", branch: "main"

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
    generate_completions_from_executable(bin/"disc", "completions")

    # Install man pages
    man1.mkpath
    system bin/"disc", "--no-color", "mangen", man1
  end

  test do
    # Verify the binary runs
    assert_match "disc #{version}", shell_output("#{bin}/disc --version")

    # Create a DSK image and list its (empty) contents
    system bin/"disc", "--no-color", "create", testpath/"test.dsk"
    output = shell_output("#{bin}/disc --no-color list #{testpath}/test.dsk")
    assert_match "0 files", output
  end
end
