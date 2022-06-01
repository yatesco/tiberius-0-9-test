This project demonstrates a possible bug in prisma/tiberius v0.9.

It fails to connect. Change the version to 0.8 in Cargo.toml and it immediately errors with incorrect credentials. 0.9 just times out.