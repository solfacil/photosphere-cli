defmodule ServiceTemplate.MixProject do
  use Mix.Project

  def project do
    [
      app: :service_template,
      version: "0.1.0",
      elixir: "~> 1.12",
      elixirc_paths: elixirc_paths(Mix.env()),
      compilers: [:gettext] ++ Mix.compilers(),
      start_permanent: Mix.env() == :prod,
      preferred_cli_env: [ci: :test],
      aliases: aliases(),
      releases: releases(),
      deps: deps(),
      name: "Template de Serviço",
      source_url: "https://github.com/solfacil/service-template",
      docs: [
        main: "Template",
        formatters: ["html"],
        extras: [
          "README.md",
          "guides/local/asdf.md",
          "guides/local/docker.md",
          "guides/local/nix.md",
          ".env-sample"
        ],
        groups_for_extras: [
          "Ambiente de desenvolvimento": ~r/guides\/local\/.*/
        ]
      ],
      test_coverage: [
        tool: ExCoveralls
      ]
    ]
  end

  def application do
    [
      mod: {ServiceTemplate.Application, []},
      extra_applications: [:logger, :runtime_tools]
    ]
  end

  # Specifies which paths to compile per environment.
  defp elixirc_paths(:test), do: ["lib", "test/support"]
  defp elixirc_paths(_), do: ["lib"]

  defp deps do
    [
      # start deps
      {:phoenix, "~> 1.6.6"},
      {:phoenix_ecto, "~> 4.4"},
      {:ecto_sql, "~> 3.6"},
      {:postgrex, ">= 0.0.0"},
      {:sentry, "~> 8.0"},
      {:phoenix_live_dashboard, "~> 0.6"},
      {:telemetry_metrics, "~> 0.6"},
      {:telemetry_poller, "~> 1.0"},
      {:guardian, "~> 2.0"},
      {:gettext, "~> 0.18"},
      {:ex_machina, "~> 2.7.0"},
      {:jason, "~> 1.2"},
      {:plug_cowboy, "~> 2.5"},
      {:credo, "~> 1.6", only: [:dev, :test], runtime: false},
      {:dialyxir, "~> 1.0", only: [:dev], runtime: false},
      {:ex_doc, "~> 0.27.3", only: [:dev], runtime: false},
      {:database, git: "git@github.com:solfacil/database.git", tag: "0.0.7"},
      {:http_client, git: "git@github.com:solfacil/http-client.git", tag: "0.0.9"},
      {:feature_flag, git: "git@github.com:solfacil/feature-flag.git", tag: "0.0.2"},
      {:hackney, "~> 1.18"},
      {:cowlib, "~> 2.11", override: true},
      {:mox, "~> 1.0", only: [:test]},
      {:spandex, "~> 3.1"},
      {:spandex_datadog, "~> 1.2"},
      {:spandex_phoenix, "~> 1.0"},
      {:spandex_ecto, "~> 0.7.0"},
      {:gen_smtp, "~> 1.0"},
      {:swoosh, "~> 1.6"},
      {:absinthe, "~> 1.6.0"},
      {:absinthe_plug, "~> 1.5.8"},
      {:absinthe_phoenix, "~> 2.0"},
      {:absinthe_relay, "~> 1.5"},
      {:protobuf, "~> 0.8.0"},
      {:grpc, github: "elixir-grpc/grpc"},
      {:google_protos, "~> 0.1"},
      {:messaging, git: "git@github.com:solfacil/messaging.git", tag: "2.0.0"},
      {:gun, "~> 2.0.0", repo: "hexpm", hex: "grpc_gun", override: true}
      {:httpoison, "~> 1.8"},
      {:prom_ex, "~> 1.6"},
      {:ex_commons, git: "git@github.com:solfacil/ex_commons.git", tag: "0.0.3", override: true},
      {:open_api_spex, "~> 3.11"},
      {:excoveralls, "~> 0.14 and >= 0.14.4", only: [:test]},
      {:logger_json, "~> 5.0", only: [:prod]}
      # end deps
    ]
  end

  defp aliases do
    [
      setup: ["deps.get", "ecto.setup"],
      "ecto.setup": ["ecto.create", "ecto.migrate", "run priv/repo/seeds.exs"],
      "ecto.reset": ["ecto.drop", "ecto.setup"],
      test: ["ecto.create --quiet", "ecto.migrate --quiet", "test"],
      "test.reset": ["ecto.drop", "test"],
      ci: ["format --check-formatted", "credo --strict", "test"]
    ]
  end

  defp releases do
    [
      service_template: [
        applications: [
          service_template: :permanent
        ]
      ]
    ]
  end
end
