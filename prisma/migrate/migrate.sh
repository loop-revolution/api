#!/bin/bash
# Clear the migrations
yarn ts-node ./prisma/migrate/clearMigrations.ts
# Migrate the database
yarn run prisma migrate save --experimental --create-db --name \"\"
yarn run prisma migrate up --create-db --experimental
# Generate the prisma code
yarn prisma generate
# # Seed the named blocks
# yarn db:named
