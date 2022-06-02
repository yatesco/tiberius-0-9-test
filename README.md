# SOLUTION
https://github.com/prisma/tiberius/issues/220

`native_tls` is enabled by default, which seems to cause some issues on macos. The following disables it and uses `rusttls`:
```toml
tiberius = { version = "0.9", default-features = false, features = ["chrono", "tokio", "rustls"] }
```

# OLD
This project demonstrates a possible bug in prisma/tiberius v0.9.

It fails to connect. Change the version to 0.8 in Cargo.toml and it immediately errors with incorrect credentials. 0.9 just times out.

Tested on macOS, latest docker, with the following running containers (although I don't think this matters at all):

```
➜  ~ docker ps
CONTAINER ID   IMAGE                                        COMMAND                  CREATED       STATUS       PORTS                    NAMES
2bed7a318033   mcr.microsoft.com/mssql/server:2019-latest   "/opt/mssql/bin/perm…"   4 hours ago   Up 4 hours   0.0.0.0:1433->1433/tcp   funny_bhabha
```

