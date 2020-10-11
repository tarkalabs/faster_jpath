require "faster_jpath/version"
require "rutie"

module FasterJpath
  class Error < StandardError; end
  Rutie.new(:faster_jpath).init 'Init_faster_jpath', __dir__
end
