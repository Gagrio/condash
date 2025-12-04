class Condash < Formula
  desc "Single-command ephemeral monitoring for Docker containers"
  homepage "https://github.com/yourusername/condash"
  version "0.1.0"
  license "Apache-2.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/Gagrio/condash/releases/download/v0.1.0/condash-macos-aarch64"
      sha256 "e3a9652f5040225a8bc1891b6a4baa6e4ea5f00e2d2ad9ed007a7422162aed4b"
    end
  end

  def install
    bin.install "condash-macos-aarch64" => "condash"
  end

  test do
    assert_match "condash", shell_output("#{bin}/condash --version")
  end
end