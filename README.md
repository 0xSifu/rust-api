# Rust Social App Backend

This project is a Rust-based backend for a social application, designed with a modular architecture inspired by NestJS. It leverages Actix Web for building the server, MongoDB for the database, and various Rust libraries for functionality such as JWT authentication and password hashing.

## Features

* Modular Architecture: Inspired by NestJS, the backend is organized into modules, each responsible for a specific domain of the application.
* Authentication: Secure user authentication using JWT and bcrypt for password hashing.
* Database Integration: MongoDB is used as the primary database for storing user data and other application data.

## Prerequisites

* Rust (latest stable version recommended)
* MongoDB
* Cargo (Rust's package manager)

## Installation

1. Clone the repository: 

    ```bash
    git clone https://github.com/yourusername/rust-social-app.git
    cd rust-social-app
    ```

2. Set Up Environment Variables: Create a .env file in the root directory and add the following environment variables:
    
    ```bash
   DATABASE_URL=mongodb+srv://<username>:<password>@cluster0.mongodb.net/social_app?retryWrites=true&w=majority
    JWT_SECRET=<your-secret-key>
    ```
   
3. Install Dependencies: Run the following command to install necessary dependencies:
   
    ```bash
    cargo build
    ```

4. Start the Server: Run the following command to start the server:
   
    ```bash
    cargo run
    ```
   
5. The server should now be running on `http://localhost:8080`.


## Project Structure

    rust-social-app/
      ├── src/
      │   ├── app/
      │   │   ├── mod.rs                # Module initialization for app-related functions
      │   │   ├── app_constants.rs      # Constants used throughout the application
      │   │   ├── app_controller.rs     # Main controllers for handling app-wide requests
      │   │   └── app_services.rs       # Services for app-wide operations
      │   │
      │   ├── common/
      │   │   └── mod.rs                # Common utilities and helper functions
      │   │
      │   ├── config/
      │   │   ├── mod.rs                # Module initialization for configuration
      │   │   └── config.rs             # Configuration settings and environment setup
      │   │
      │   ├── decorators/               # Custom decorators (empty for now)
      │   │
      │   ├── guards/
      │   │   ├── mod.rs                # Module initialization for guards
      │   │   └── jwt_guard.rs          # JWT guard for route protection
      │   │
      │   ├── interceptors/
      │   │   └── mod.rs                # Module initialization for interceptors (empty for now)
      │   │
      │   ├── models/
      │   │   ├── schema/               # Schema definitions for MongoDB collections
      │   │   └── migration/            # Migration scripts for database setup
      │   │
      │   ├── modules/
      │   │   ├── auth/                 # Authentication module
      │   │   ├── follow/               # Follow module
      │   │   ├── story/                # Story module
      │   │   ├── user/                 # User module
      │   │   └── mod.rs                # Module initialization for all feature modules
      │   │
      │   ├── strategies/               # Strategies (empty for now)
      │   │
      │   └── main.rs                   # Entry point of the application
      │
      ├── Cargo.toml                    # Cargo configuration file with dependencies
      └── .env                          # Environment variables file

   ```