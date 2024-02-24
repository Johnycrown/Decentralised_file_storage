**Decentralized File Storage System

This project implements a decentralized file storage system using the Internet Computer platform. It allows users to upload files, download files, and create directories within a decentralized environment.

**Overview
**
The decentralized file storage system is implemented using Rust programming language and leverages the Internet Computer's Canister Development Kit (CDK) for communication and interaction with the platform.

The system consists of the following components:

**File struct**: Represents a file with a name and content.
**Directory struct**: Represents a directory with a name, containing files, and subdirectories.
**FileStorage trait**: Defines methods for uploading files, downloading files, and creating directories.
**DecentralizedFileStorage** struct: Implements the FileStorage trait for decentralized file storage.

**Usage**

**Uploading a File**
To upload a file, use the upload_file method of the FileStorage trait. Provide the directory path, file name, and file content as parameters. This method returns a Result indicating success or failure.

**Downloading a File**
To download a file, use the download_file method of the FileStorage trait. Provide the file path as a parameter. This method returns a Result containing the file content if the file exists, otherwise an error indicating file not found.

**Creating a Directory**
To create a directory, use the create_directory method of the FileStorage trait. Provide the parent directory path and the directory name as parameters. This method returns a Result indicating success or failure.

**Implementation Details**

The DecentralizedFileStorage struct maintains the root directory of the file storage system.
Files are stored as instances of the File struct within directories.
Directories are stored as instances of the Directory struct, which may contain files and subdirectories.
The **upload_file**, **download_file**, and **create_directory** methods of the **FileStorage** trait are implemented to interact with the file storage system.
Queries for uploading, downloading, and creating directories are exposed through export functions (canister_query functions) for interaction with the Internet Computer platform.

**Getting Started**

To get started with using the decentralized file storage system, follow these steps:

1. Ensure that you have Rust and the Internet Computer development environment set up.
2. Clone the repository containing the project code.
3. Build and deploy the canister containing the file storage system to the Internet Computer.
4. Use the exported functions to interact with the file storage system.
