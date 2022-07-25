defmodule NifIdenticonTest do
  use ExUnit.Case
  doctest NifIdenticon

  test "Generate identicon binary" do
    identicon_bitstring = NifIdenticon.generate_binary("hello")
    assert is_bitstring(identicon_bitstring) == true
  end

  test "Generate identicon base64" do
    identicon_base64 = NifIdenticon.generate_base64("hello")
    assert is_binary(identicon_base64) == true
  end
end
