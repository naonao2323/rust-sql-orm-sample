env "local" {
    src = "file://migrations"
    url = "mysql://root:root@localhost:3305/app"
    dev = "docker://mysql/8/dev"
    schemas = [ "app" ]
    migration {
      dir = "file://migrations"
    }
    diff {
        // By default, indexes are not created or dropped concurrently.
        concurrent_index {
        create = true
        drop   = true
        }
    }
}

