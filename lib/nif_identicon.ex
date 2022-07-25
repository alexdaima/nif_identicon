defmodule NifIdenticon do
  @moduledoc """
  Documentation for `NifIdenticon`.
  """

  use Rustler,
    otp_app: :nif_identicon,
    crate: "nif_identicon",
    skip_compilation?: false

  defp err do
    throw(NifNotLoadedError)
  end

  @doc "Generate a png binary of the identicon image"
  @spec generate_binary(String.t()) :: binary
  def generate_binary(_name), do: err()

  @doc "Generate a base64 string of the identicon image"
  @spec generate_base64(String.t()) :: String.t()
  def generate_base64(_name), do: err()
end
