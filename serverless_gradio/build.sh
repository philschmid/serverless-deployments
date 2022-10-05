docker buildx build \
    --platform=linux/arm64 \
    --output ./build \
    --file Dockerfile \
    .