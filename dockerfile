# Use a base Linux distribution as the starting point
FROM ubuntu:latest
#FROM alpine:latest
# Set non-interactive mode for installations
#ARG DEBIAN_FRONTEND=noninteractive
ENV RUST_BACKTRACE=1
RUN sed -i 's/htt[p|ps]:\/\/archive.ubuntu.com\/ubuntu\//mirror:\/\/mirrors.ubuntu.com\/mirrors.txt/g' /etc/apt/sources.list
#Splitin for better caching

RUN apt-get update 
RUN apt-get install -y ca-certificates
RUN apt-get install -y libglu1
RUN apt-get install -y tar wget bzip2
RUN apt-get install -y fuse  
RUN apt-get install -y libglu1-mesa
RUN apt-get install -y libpangoxft-1.0
RUN apt-get install -y libegl1 
RUN apt-get install -y libgl1
RUN apt-get install -y libgl1-amber-dri
RUN apt-get install -y libgl1-mesa-dri
RUN apt-get install -y libglapi-mesa
RUN apt-get install -y libatk-bridge2.0-0
RUN apt-get install -y libatk1.0-0
RUN apt-get install -y libatk1.0-data
RUN apt-get install -y libgdk-pixbuf2.0-0
RUN apt-get install -y libgdk-pixbuf2.0-bin
RUN apt-get install -y libgdk-pixbuf2.0-common
RUN apt-get install -y libcairo-gobject2
RUN apt-get install -y libcairo2
RUN apt-get install -y libpangocairo-1.0-0
RUN apt-get install -y libgtk2.0-0
RUN apt-get install -y libwayland-egl1
RUN apt-get install -y grep
WORKDIR /root
# Download latest prusa-slicer release, usually in github.
RUN wget https://github.com/prusa3d/PrusaSlicer/releases/download/version_2.6.1/PrusaSlicer-2.6.1+linux-x64-GTK2-202309060801.tar.bz2
# untar specific file, in this case is prusaversion/bin/prusa-slicer
RUN tar -xjf PrusaSlicer-2.6.1+linux-x64-GTK2-202309060801.tar.bz2
# Move the prusa-slicer binary to /usr/bin
RUN mv PrusaSlicer-2.6.1+linux-x64-GTK2-202309060801/bin/prusa-slicer /usr/bin
# Remove the downloaded archive
RUN rm -rf PrusaSlicer-2.6.1+linux-x64-GTK2-202309060801.tar.bz2
# Remove the downloaded folder
RUN rm -rf PrusaSlicer-2.6.1+linux-x64-GTK2-202309060801
# copy slicer file from directory to container
ADD slicer /root/
# Set the working directory to /root

# I want to execute slicer file which is a web application, its in root
CMD ["./slicer"]
# Expose port 3080
EXPOSE 3080
# Clean up
RUN apt-get clean && \
  rm -rf /var/lib/apt/lists/*
# Set the entry point to run PrusaSlicer
