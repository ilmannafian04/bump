variable "version" {
  type = string
}

job "bump" {
  datacenters = ["ln-sg"]

  group "bump" {
    network {
      port "http" {}
    }

    service {
      name = "bump"
      port = "http"
      tags = [
        "traefik.enable=true",
        "traefik.http.routers.bump.rule=Host(`bump.ilmannfn.com`)",
        "traefik.http.routers.bump.entrypoints=websecure",
        "traefik.http.routers.bump.tls=true",
        "traefik.http.routers.bump.tls.certResolver=cloudflareResolver",
      ]

      check {
        name = "Bump HTTP Check"
        type = "http"
        port = "http"
        path = "/health/live"

        interval = "30s"
        timeout  = "10s"
      }
    }

    task "bump" {
      driver = "docker"

      config {
        image = "ilmannafian/bump:${var.version}"
        ports = ["http"]
      }

      env {
        HOST = "0.0.0.0"
        PORT = NOMAD_PORT_http

        RUST_LOG = "info"
      }

      resources {
        cpu    = 10
        memory = 10
      }
    }
  }
}
