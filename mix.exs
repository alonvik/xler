defmodule Xler.MixProject do
  use Mix.Project

  def project do
    [
      app: :xler,
      version: project_version(),
      elixir: "~> 1.8",
      start_permanent: Mix.env() == :prod,
      description: description(),
      package: package(),
      deps: deps(),
      docs: docs()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application() do
    [
      extra_applications: [:logger]
    ]
  end

  # Returns the current project version from VERSION
  defp project_version do
    "VERSION"
    |> File.read!()
    |> String.trim()
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps() do
    [
      {:rustler, ">= 0.34.0"},
      {:rustler_precompiled, "~> 0.8"},
      {:ex_doc, ">= 0.0.0", only: :dev}
    ]
  end

  defp docs do
    [
      source_ref: "master",
      main: "Xler",
      canonical: "http://hexdocs.pm/xler",
      source_url: "https://github.com/jnylen/xler",
      extras: [
        "README.md"
      ]
    ]
  end

  defp description() do
    "A rust-based Excel parser."
  end

  defp package() do
    [
      files: ~w(lib .formatter.exs mix.exs README* LICENSE*
                native checksum-*.exs),
      licenses: ["MIT"],
      links: %{"Github" => "https://github.com/jnylen/xler"}
    ]
  end
end
