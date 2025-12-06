# Publishing pino-rs to npm

This guide walks you through publishing the pino-rs Node.js bindings to npm.

## Prerequisites

1. **Node.js** (v10 or later)
2. **Rust** (1.70 or later)
3. **npm account** - Create one at https://www.npmjs.com/signup
4. **@napi-rs/cli** - Will be installed as dev dependency

## Step 1: Prepare the Package

### 1.1 Update package.json

First, update the package name and repository in `pino-node/package.json`:

```json
{
  "name": "@yourusername/pino-rs",  // or just "pino-rs" if available
  "version": "0.1.0",
  "repository": {
    "type": "git",
    "url": "https://github.com/yourusername/pino-rs"
  }
}
```

### 1.2 Choose a package name

Check if your desired name is available:

```bash
npm view pino-rs
# If not found, the name is available
```

## Step 2: Install Dependencies

```bash
cd pino-node
npm install
```

This will install `@napi-rs/cli` which handles cross-platform builds.

## Step 3: Build for Your Current Platform

### 3.1 Build the native module

```bash
npm run build
```

This creates the `.node` file (native binary) for your current platform.

### 3.2 Test locally

```bash
node test.js
```

Verify all tests pass before publishing.

## Step 4: Login to npm

```bash
npm login
```

Enter your npm username, password, and email.

## Step 5: Publishing Options

You have two options for publishing:

### Option A: Single Platform (Quick Start)

This publishes only for your current platform (macOS, Linux, or Windows).

```bash
npm publish --access public
```

**Note**: This is fine for testing, but users on other platforms won't be able to use your package.

### Option B: Multi-Platform (Recommended)

For production, you should support multiple platforms.

#### Manual Multi-Platform Build

You'll need access to different machines or use cross-compilation:

```bash
# On macOS (Intel)
npm run build -- --target x86_64-apple-darwin

# On macOS (Apple Silicon)
npm run build -- --target aarch64-apple-darwin

# On Linux (x64)
npm run build -- --target x86_64-unknown-linux-gnu

# On Windows (x64)
npm run build -- --target x86_64-pc-windows-msvc
```

Then publish:

```bash
npm run prepublishOnly
npm publish --access public
```

## Step 6: Automated Publishing with GitHub Actions (Recommended)

Create `.github/workflows/publish.yml`:

```yaml
name: Publish to npm

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      fail-fast: false
      matrix:
        settings:
          - host: macos-latest
            target: x86_64-apple-darwin
            build: npm run build -- --target x86_64-apple-darwin
          - host: macos-latest
            target: aarch64-apple-darwin
            build: npm run build -- --target aarch64-apple-darwin
          - host: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            build: npm run build -- --target x86_64-unknown-linux-gnu
          - host: windows-latest
            target: x86_64-pc-windows-msvc
            build: npm run build -- --target x86_64-pc-windows-msvc

    runs-on: ${{ matrix.settings.host }}

    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: 18
          registry-url: 'https://registry.npmjs.org'

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.settings.target }}

      - name: Install dependencies
        run: cd pino-node && npm install

      - name: Build
        run: cd pino-node && ${{ matrix.settings.build }}

      - name: Upload artifacts
        uses: actions/upload-artifact@v4
        with:
          name: bindings-${{ matrix.settings.target }}
          path: pino-node/*.node

  publish:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: 18
          registry-url: 'https://registry.npmjs.org'

      - name: Download all artifacts
        uses: actions/download-artifact@v4
        with:
          path: pino-node/artifacts

      - name: Move artifacts
        run: cd pino-node && npm run artifacts

      - name: Publish to npm
        run: cd pino-node && npm publish --access public
        env:
          NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
```

### Set up GitHub Actions:

1. Get your npm token:
   ```bash
   npm token create
   ```

2. Add it to GitHub:
   - Go to your repo → Settings → Secrets → New repository secret
   - Name: `NPM_TOKEN`
   - Value: your token

3. Create a git tag and push:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

## Step 7: Update Version for Future Releases

```bash
cd pino-node
npm version patch  # 0.1.0 -> 0.1.1
npm version minor  # 0.1.0 -> 0.2.0
npm version major  # 0.1.0 -> 1.0.0
```

## Step 8: Verify Publication

After publishing, verify your package:

```bash
npm view @yourusername/pino-rs
```

Test installation in a new directory:

```bash
mkdir test-install
cd test-install
npm init -y
npm install @yourusername/pino-rs
```

## Quick Publishing Checklist

- [ ] Package name is available or you own it
- [ ] `package.json` has correct name, version, and repository
- [ ] All tests pass (`node test.js`)
- [ ] Built successfully (`npm run build`)
- [ ] Logged into npm (`npm login`)
- [ ] Choose scope: `--access public` for public packages
- [ ] Publish: `npm publish --access public`
- [ ] Verify: `npm view your-package-name`

## Important Notes

### Package Scope

- **Scoped package** (`@username/pino-rs`): Always available, requires `--access public`
- **Unscoped package** (`pino-rs`): Might be taken, no scope needed

### Platform Support

If you publish with only one platform's binary:
- Users on that platform can install and use it
- Users on other platforms will get installation errors

For production use, implement multi-platform builds via CI/CD.

### Pre-publish Test

Always test before publishing:

```bash
# Build
npm run build

# Test
node test.js

# Pack to see what will be published
npm pack

# Check the contents
tar -xzf *.tgz
ls package/
```

## Troubleshooting

### "Cannot find module '*.node'"

Make sure the build completed successfully and the `.node` file exists.

### "EAUTH" error

Run `npm login` again or check your npm token.

### "403 Forbidden"

- For scoped packages, add `--access public`
- Check if package name is already taken
- Verify you have permission to publish

### Platform-specific issues

Different platforms need different binaries. Use GitHub Actions for multi-platform support.

## Next Steps

1. Set up automated publishing with GitHub Actions
2. Add platform badges to README
3. Create releases on GitHub
4. Monitor npm download stats
5. Respond to issues and maintain the package

## Resources

- [npm Documentation](https://docs.npmjs.com/cli/v9/commands/npm-publish)
- [napi-rs Documentation](https://napi.rs/)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
