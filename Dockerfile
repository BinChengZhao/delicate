#This is a sample Image 
FROM rust:alpine 
COPY ./README.md .
RUN ["cat", "./README.md"] 