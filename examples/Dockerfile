# Use the official Rilot image
FROM apprilot/rilot-core:beta2

# Copy your config file
COPY config/config.json /app/config/config.json

# Copy your WASM files if needed
COPY target/wasm32-wasip1/release/*.wasm /app/wasm/

# Set environment variables
ENV RILOT_HOST=0.0.0.0
ENV RILOT_PORT=3000
ENV RUST_LOG=debug

# # Run Rilot with your config
ENTRYPOINT ["/usr/local/bin/rilot"]
CMD ["/app/config/config.json"]
