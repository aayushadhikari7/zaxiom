//! Project Detection
//!
//! Detects project type based on files in the current directory.
//! Provides smart project context awareness.

#![allow(dead_code)]

use std::path::Path;

/// Detected project type
#[derive(Clone, Debug, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub enum ProjectType {
    /// Rust project (Cargo.toml)
    Rust,
    /// Node.js project (package.json)
    Node,
    /// Python project (requirements.txt, setup.py, pyproject.toml)
    Python,
    /// Go project (go.mod)
    Go,
    /// Java/Maven project (pom.xml)
    Maven,
    /// Java/Gradle project (build.gradle)
    Gradle,
    /// .NET project (*.csproj, *.sln)
    DotNet,
    /// Ruby project (Gemfile)
    Ruby,
    /// PHP/Composer project (composer.json)
    PHP,
    /// Docker project (Dockerfile)
    Docker,
    /// Git repository
    Git,
    /// Unknown/no project
    Unknown,
}

impl ProjectType {
    /// Get icon for the project type
    pub fn icon(&self) -> &'static str {
        match self {
            ProjectType::Rust => "🦀",
            ProjectType::Node => "📦",
            ProjectType::Python => "🐍",
            ProjectType::Go => "🐹",
            ProjectType::Maven | ProjectType::Gradle => "☕",
            ProjectType::DotNet => "🔷",
            ProjectType::Ruby => "💎",
            ProjectType::PHP => "🐘",
            ProjectType::Docker => "🐳",
            ProjectType::Git => "",
            ProjectType::Unknown => "📁",
        }
    }

    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            ProjectType::Rust => "Rust",
            ProjectType::Node => "Node.js",
            ProjectType::Python => "Python",
            ProjectType::Go => "Go",
            ProjectType::Maven => "Maven",
            ProjectType::Gradle => "Gradle",
            ProjectType::DotNet => ".NET",
            ProjectType::Ruby => "Ruby",
            ProjectType::PHP => "PHP",
            ProjectType::Docker => "Docker",
            ProjectType::Git => "Git",
            ProjectType::Unknown => "",
        }
    }

    /// Get suggested commands for this project type
    pub fn suggested_commands(&self) -> Vec<(&'static str, &'static str)> {
        match self {
            ProjectType::Rust => vec![
                ("cargo build", "Build the project"),
                ("cargo run", "Run the project"),
                ("cargo test", "Run tests"),
                ("cargo check", "Check for errors"),
                ("cargo fmt", "Format code"),
                ("cargo clippy", "Run linter"),
            ],
            ProjectType::Node => vec![
                ("npm install", "Install dependencies"),
                ("npm run dev", "Start dev server"),
                ("npm run build", "Build for production"),
                ("npm test", "Run tests"),
                ("npx", "Run package binary"),
            ],
            ProjectType::Python => vec![
                ("python", "Run Python"),
                ("pip install -r requirements.txt", "Install deps"),
                ("pytest", "Run tests"),
                ("python -m venv venv", "Create virtualenv"),
                ("source venv/bin/activate", "Activate venv"),
            ],
            ProjectType::Go => vec![
                ("go build", "Build the project"),
                ("go run .", "Run the project"),
                ("go test ./...", "Run tests"),
                ("go mod tidy", "Clean up modules"),
            ],
            ProjectType::Maven => vec![
                ("mvn clean install", "Build project"),
                ("mvn test", "Run tests"),
                ("mvn package", "Create JAR/WAR"),
            ],
            ProjectType::Gradle => vec![
                ("./gradlew build", "Build project"),
                ("./gradlew test", "Run tests"),
                ("./gradlew run", "Run project"),
            ],
            ProjectType::DotNet => vec![
                ("dotnet build", "Build project"),
                ("dotnet run", "Run project"),
                ("dotnet test", "Run tests"),
            ],
            ProjectType::Ruby => vec![
                ("bundle install", "Install gems"),
                ("rails server", "Start Rails server"),
                ("rake test", "Run tests"),
            ],
            ProjectType::PHP => vec![
                ("composer install", "Install dependencies"),
                ("php artisan serve", "Start Laravel server"),
                ("phpunit", "Run tests"),
            ],
            ProjectType::Docker => vec![
                ("docker build -t app .", "Build image"),
                ("docker run app", "Run container"),
                ("docker-compose up", "Start services"),
            ],
            ProjectType::Git => vec![
                ("git status", "Check status"),
                ("git diff", "View changes"),
                ("git log --oneline", "View history"),
            ],
            ProjectType::Unknown => vec![],
        }
    }
}

/// Detect the project type for a given directory
pub fn detect_project(path: &Path) -> ProjectType {
    // Check for various project files
    if path.join("Cargo.toml").exists() {
        return ProjectType::Rust;
    }
    if path.join("package.json").exists() {
        return ProjectType::Node;
    }
    if path.join("requirements.txt").exists()
        || path.join("setup.py").exists()
        || path.join("pyproject.toml").exists()
        || path.join("Pipfile").exists()
    {
        return ProjectType::Python;
    }
    if path.join("go.mod").exists() {
        return ProjectType::Go;
    }
    if path.join("pom.xml").exists() {
        return ProjectType::Maven;
    }
    if path.join("build.gradle").exists() || path.join("build.gradle.kts").exists() {
        return ProjectType::Gradle;
    }
    if has_extension(path, "csproj") || has_extension(path, "sln") {
        return ProjectType::DotNet;
    }
    if path.join("Gemfile").exists() {
        return ProjectType::Ruby;
    }
    if path.join("composer.json").exists() {
        return ProjectType::PHP;
    }
    if path.join("Dockerfile").exists() || path.join("docker-compose.yml").exists() {
        return ProjectType::Docker;
    }
    if path.join(".git").exists() {
        return ProjectType::Git;
    }

    ProjectType::Unknown
}

/// Check if directory contains files with a specific extension
fn has_extension(path: &Path, ext: &str) -> bool {
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Some(e) = entry.path().extension() {
                if e.to_string_lossy().eq_ignore_ascii_case(ext) {
                    return true;
                }
            }
        }
    }
    false
}

/// Get project info string for status bar
pub fn get_project_info(path: &Path) -> Option<String> {
    let project = detect_project(path);
    if project == ProjectType::Unknown {
        return None;
    }

    Some(format!("{} {}", project.icon(), project.name()))
}
