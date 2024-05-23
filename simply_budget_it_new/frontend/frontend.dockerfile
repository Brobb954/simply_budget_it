# Stage 1: Builder
FROM oven/bun:1 AS base
WORKDIR /frontend

# Install dependencies into temp directory to create a cache layer
FROM base AS  install
RUN mkdir -p /temp/dev
COPY package.json bun.lockb /temp/dev/
RUN cd /temp/dev && bun install --frozen-lockfile

# Create a production cache layer
RUN mkdir -p /temp/prod
COPY package.json bun.lockb /temp/prod/
RUN cd /temp/prod && bun install --frozen-lockfile --production; 

# Copy node_modules from temp directory
# then copy the rest of the files
FROM base AS prerelase
COPY --from=install /temp/dev/node_modules ./node_modules
COPY . .

# Test and build the application
ENV NODE_ENV=production
RUN bun test
RUN bun run build

# copy production dependencies and source code into final image
FROM base AS release
COPY --from=install /temp/prod/node_modules ./node_modules
COPY --from=prerelase  /frontend/.next/standalone ./
COPY --from=prerelase  /frontend/.next/static ./.next/static

USER bun

EXPOSE 3000

ENV PORT 3000

CMD ["bun", "run", "start"]