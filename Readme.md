# Bamboo Timesheet Bot

Simple CLI program to automatically fill BambooHR timesheets 

## Features

- filling monday-friday timesheets with 8am - 4pm hours
- sending entries in batch (whole month) or individually
- vacation and bank holidays supported

## Usage

### Install Rust

> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

### Run bot


You can optionally create .env file and add env variables there:

```
API_KEY=<BAMBOO_API_KEY> 
EMPLOYEE_ID=<BAMBOO_EMPLOYEE_ID> 
COMPANY=<COMPANY_DOMAIN>
```

Then in project directory run in console: 
> cargo run

Alternatively if you don't have .env file, you can pass variables directly:

> cargo run -- --api_key=<BAMBOO_API_KEY> --employee_id=<BAMBOO_EMPLOYEE_ID> --company=<COMPANY_DOMAIN>

Follow instructions provided by the program.

Don't worry, it won't send anything without your confirmation!