defmodule Xler.Native do
  @moduledoc false

  version = Mix.Project.config()[:version]

  use RustlerPrecompiled,
    otp_app: :xler,
    crate: "xler_native",
    base_url: "https://github.com/pjullrich/xler/releases/download/v#{version}",
    force_build: System.get_env("RUSTLER_FORCE_BUILD") in ["1", "true"],
    targets:
      Enum.uniq(["aarch64-unknown-linux-musl" | RustlerPrecompiled.Config.default_targets()]),
    version: version

  def parse(_filename, _worksheet), do: error()
  def worksheets(_filename), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
