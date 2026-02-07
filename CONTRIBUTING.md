# Contributing to Multi-VM AI Agent Workspace Tool

Thank you for your interest in contributing! We welcome developers, designers, documentation writers, and community members. This guide will help you get started.

## Code of Conduct

We are committed to providing a welcoming and inclusive environment for all contributors. Please be respectful, constructive, and supportive of others.

**Core Principles**:
- Be kind and respectful to all community members
- Welcome diverse perspectives and experiences
- Focus on the work, not the person
- Assume good intent and ask clarifying questions
- Report violations to the maintainers

## Getting Started

### Prerequisites

Before you can contribute code, you'll need:

- **Rust** (latest stable) — [Install Rust](https://rustup.rs/)
- **Node.js** (v18+) — [Install Node.js](https://nodejs.org/)
- **Git** — [Install Git](https://git-scm.com/)

### Development Environment Setup

**TBD** — Detailed setup instructions coming soon. For now:

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/multivm-workspace-tool.git
   cd multivm-workspace-tool
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Build the project:
   ```bash
   npm run build
   ```

4. Run tests:
   ```bash
   npm test
   ```

## How to Contribute

### Reporting Bugs

Found a bug? Please create a GitHub issue with:

- **Title**: Clear, concise description of the bug
- **Description**: What happened, what you expected to happen
- **Steps to Reproduce**: Exact steps to reproduce the issue
- **Environment**: OS, Node.js version, Rust version
- **Screenshots**: If applicable

### Requesting Features

Have an idea? Create a GitHub discussion or issue with:

- **Title**: Clear feature description
- **Use Case**: Why you need this feature
- **Proposed Solution**: How you'd like it to work
- **Alternatives**: Other approaches you've considered

### Submitting Code

1. **Fork the repository** on GitHub
2. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. **Make your changes** following the code style guidelines (see below)
4. **Write tests** for your changes
5. **Commit with clear messages**:
   ```bash
   git commit -m "feat: add new feature description"
   ```
6. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```
7. **Create a Pull Request** with a clear description

### Pull Request Process

1. Update documentation if you've changed functionality
2. Add tests for new features
3. Ensure all tests pass: `npm test`
4. Ensure code follows style guidelines: `npm run lint`
5. Request review from maintainers
6. Address feedback and push updates
7. Maintainers will merge when approved

## Code Style and Conventions

**TBD** — Detailed code style guidelines coming soon. For now:

- Follow standard Rust conventions (use `rustfmt`)
- Follow standard JavaScript/TypeScript conventions (use `prettier`)
- Write clear, descriptive variable and function names
- Add comments for complex logic
- Keep functions small and focused

### Commit Message Format

Use conventional commits:

```
type(scope): subject

body

footer
```

**Types**: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

**Example**:
```
feat(workset): add workset profile management

Implement CRUD operations for Workset profiles with JSON persistence.
Allows users to save and restore SSH configs, project paths, and grid layouts.

Closes #42
```

## Testing

**TBD** — Testing guidelines coming soon. For now:

- Write tests for all new features
- Ensure existing tests still pass
- Aim for >80% code coverage

## Documentation

Help improve our documentation:

- Fix typos and clarify confusing sections
- Add examples and use cases
- Improve architecture diagrams
- Translate documentation to other languages

## Community

- **GitHub Issues**: Report bugs and request features
- **GitHub Discussions**: Ask questions and share ideas
- **Discord** (coming soon): Real-time community chat

## Questions?

- Check the [documentation](./docs/README.md)
- Search existing GitHub issues
- Create a new GitHub discussion
- Reach out to maintainers

## Recognition

Contributors will be recognized in:
- Project README
- Release notes
- Contributors page (coming soon)

Thank you for making this project better! 🎉

---

**Last Updated**: February 7, 2026
