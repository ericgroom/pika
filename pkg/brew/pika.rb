# Documentation: https://docs.brew.sh/Formula-Cookbook
#                https://rubydoc.brew.sh/Formula
class Pika < Formula
  desc "A meme text formatter"
  homepage "https://github.com/ericgroom/pika/"
  url "https://github.com/ericgroom/pika/releases/download/0.1/darwin-0.1.tar.gz"
  sha256 "794b06599e4c0967f97ca55389d953a10404dca1e71a2d25fffc7d0855db10b7"

  def install
    bin.install "pika"
  end
end
