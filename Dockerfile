# This Dockerfile needs to be built outside, before building this.
# This is an inconvenience, and so it should only be built in CI/CD.
# yarn install --production && yarn build

FROM node:slim

# Copy the built nexus coe
WORKDIR /api
COPY ./.nexus/build /api

EXPOSE 4000
CMD node index.js
