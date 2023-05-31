# Base image
FROM mongo:latest

# Install Python and required packages
RUN apt-get update && apt-get install -y python3 python3-pip

# Set working directory
WORKDIR /app

# Copy the Flask app files to the working directory
COPY . .

# Install the required Python packages
RUN pip3 install --no-cache-dir -r requirements.txt

# Expose the Flask app port
EXPOSE 8080


# Start the Flask app
CMD /bin/sh entry.sh
