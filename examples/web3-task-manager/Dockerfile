FROM python:3.9-slim

WORKDIR /app

# Install dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy application
COPY app.py .

# Expose port for Web3 Task Manager API
EXPOSE 8000

# Environment variables for BPI integration
ENV BPI_VM_URL=http://host.docker.internal:7777
ENV BPCI_BRIDGE_URL=http://host.docker.internal:8545
ENV STORAGE_URL=http://host.docker.internal:27017
ENV ORCHESTRATOR_URL=http://host.docker.internal:9090

# Run the Web3 Task Manager
CMD ["python", "app.py"]
