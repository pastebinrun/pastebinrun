# Bug reports

Use [the bug tracker](https://gitlab.com/pastebinrun/pastebinrun/issues).
If it's a security issue, ensure "This issue is confidential and should
only be visible to team members with at least Reporter access" is checked.

# Development

You need to have Node.js 12 and Rust 1.39 or higher installed. It's
recommended to use [`rustup`](https://rustup.rs/) for managing Rust
versions.

1. Install Node.js packages.

   ```sh
   npm install
   ```

2. Run webpack. This will automatically rebuild JavaScript files on
   every TypeScript change.

   ```sh
   node_modules/.bin/webpack --watch
   ```

3. Run the project. Ensure that `DATABASE_URL` is set to a new
   PostgreSQL database. Migrations will automatically run on it.
   If you want to test sandbox, you will also need to set up
   `SANDBOX_URL` environment variable.

   For instance, if you have created a `pastebinrun` database, you
   can run pastebin.run as follows.

   ```sh
   env DATABASE_URL=postgresql:///pastebinrun cargo run
   ```

4. If a contribution adds new features, it's strongly encouraged
   to add tests for those features.

5. Reformat your code before submitting merge request. Use latest
   Rust stable for reformatting the code.

   ```sh
   cargo fmt
   ```
