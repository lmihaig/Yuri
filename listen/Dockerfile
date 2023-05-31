# Use an official Python runtime as the base image
FROM python:3.9-slim

# Set the working directory in the container
WORKDIR /app

# Copy the requirements file to the container
COPY requirements.txt .

# Install the Python dependencies
RUN pip install --no-cache-dir -r requirements.txt

# Copy the rest of the application's source code to the container
COPY . .

# Expose the port that your app listens on (if applicable)
EXPOSE 8000

# Set the environment variables for MongoDB
ENV MONGO_HOST=mongodb
ENV MONGO_PORT=27017

# Install MongoDB client (if needed)
RUN apt-get update && apt-get install -y mongodb-clients

# Start MongoDB and run the Python app on container startup
CMD ["bash", "-c", "service mongodb start && python app.py"]
