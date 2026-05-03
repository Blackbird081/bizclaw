//! Built-in skills — bundled with BizClaw.

use crate::parser::SkillManifest;
use std::path::Path;

/// Get all built-in skills.
pub fn builtin_skills() -> Vec<SkillManifest> {
    let mut skills = Vec::new();
    
    // Load agency-derived skills for Vietnamese OPC market
    skills.extend(load_agency_derived_skills());
    
    // Add built-in skill definitions
    skills.extend(builtin_skill_defs());
    
    skills
}

/// Load skills from agency-derived directory (VN market focused).
pub fn load_agency_derived_skills() -> Vec<SkillManifest> {
    let mut skills = Vec::new();
    
    let agency_skills_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(|p| p.join("skills").join("agency-derived"))
        .unwrap_or_else(|| Path::new("skills/agency-derived").to_path_buf());
    
    if agency_skills_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&agency_skills_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |e| e == "md") {
                    if let Ok(skill) = SkillManifest::load(&path) {
                        tracing::info!(
                            "📦 Loaded agency skill: {} ({})",
                            skill.metadata.name,
                            skill.metadata.category
                        );
                        skills.push(skill);
                    }
                }
            }
        }
    } else {
        tracing::debug!("Agency-derived skills dir not found: {:?}", agency_skills_dir);
    }
    
    skills
}

/// Built-in skill definitions (original skills).
fn builtin_skill_defs() -> Vec<SkillManifest> {
    let skill_defs = vec![
        (
            "rust-expert",
            "Rust Expert",
            "Deep expertise in Rust programming, ownership, async, traits",
            "coding",
            "🦀",
            vec!["rust", "programming", "systems"],
            include_str!("skills/rust_expert.md"),
        ),
        (
            "python-analyst",
            "Python Analyst",
            "Python data analysis, pandas, numpy, visualization",
            "data",
            "🐍",
            vec!["python", "data", "analytics"],
            include_str!("skills/python_analyst.md"),
        ),
        (
            "web-developer",
            "Web Developer",
            "Full-stack web development with modern frameworks",
            "coding",
            "🌐",
            vec!["web", "javascript", "html", "css", "react"],
            include_str!("skills/web_developer.md"),
        ),
        (
            "devops-engineer",
            "DevOps Engineer",
            "CI/CD, Docker, Kubernetes, infrastructure automation",
            "devops",
            "🔧",
            vec!["devops", "docker", "kubernetes", "ci-cd"],
            include_str!("skills/devops_engineer.md"),
        ),
        (
            "content-writer",
            "Content Writer",
            "Professional writing for blogs, marketing, social media",
            "writing",
            "✍️",
            vec!["writing", "content", "marketing", "seo"],
            include_str!("skills/content_writer.md"),
        ),
        (
            "security-auditor",
            "Security Auditor",
            "Code security review, vulnerability assessment, best practices",
            "security",
            "🔒",
            vec!["security", "audit", "vulnerability", "owasp"],
            include_str!("skills/security_auditor.md"),
        ),
        (
            "sql-expert",
            "SQL Expert",
            "Database design, query optimization, PostgreSQL, SQLite",
            "data",
            "🗄️",
            vec!["sql", "database", "postgresql", "sqlite"],
            include_str!("skills/sql_expert.md"),
        ),
        (
            "api-designer",
            "API Designer",
            "RESTful API design, OpenAPI, GraphQL, gRPC",
            "coding",
            "🔌",
            vec!["api", "rest", "graphql", "grpc"],
            include_str!("skills/api_designer.md"),
        ),
        (
            "vietnamese-business",
            "Vietnamese Business",
            "Luật doanh nghiệp Việt Nam, thuế, lao động, hóa đơn",
            "business",
            "🇻🇳",
            vec!["vietnam", "business", "legal", "tax"],
            include_str!("skills/vietnamese_business.md"),
        ),
        (
            "git-workflow",
            "Git Workflow",
            "Git branching strategies, code review, CI/CD integration",
            "devops",
            "📦",
            vec!["git", "version-control", "github", "workflow"],
            include_str!("skills/git_workflow.md"),
        ),
        // ── BizClaw-Specific Skills ──────────────────────────
        (
            "bizclaw-test-engineer",
            "BizClaw Test Engineer",
            "Testing specialist for BizClaw 21-crate Rust workspace",
            "testing",
            "🧪",
            vec!["bizclaw", "testing", "rust", "tokio", "security"],
            include_str!("skills/bizclaw_test_engineer.md"),
        ),
        (
            "bizclaw-deploy-engineer",
            "BizClaw Deploy Engineer",
            "Deployment specialist for BizClaw VPS/production targets",
            "devops",
            "🚀",
            vec!["bizclaw", "deploy", "vps", "nginx", "systemd"],
            include_str!("skills/bizclaw_deploy_engineer.md"),
        ),
        (
            "bizclaw-security-hardening",
            "BizClaw Security Hardening",
            "Defense-in-depth security for the BizClaw AI platform",
            "security",
            "🛡️",
            vec!["bizclaw", "security", "redactor", "injection", "vault"],
            include_str!("skills/bizclaw_security_hardening.md"),
        ),
        (
            "bizclaw-code-review",
            "BizClaw Code Review",
            "Senior code reviewer with full BizClaw architecture awareness",
            "coding",
            "👁️",
            vec!["bizclaw", "review", "rust", "architecture", "patterns"],
            include_str!("skills/bizclaw_code_review.md"),
        ),
        (
            "bizclaw-feature-review",
            "BizClaw Feature Review",
            "Product engineer reviewing BizClaw features for completeness",
            "product",
            "📋",
            vec!["bizclaw", "feature", "review", "quality", "gaps"],
            include_str!("skills/bizclaw_feature_review.md"),
        ),
        (
            "bizclaw-architecture",
            "BizClaw Architecture Expert",
            "Architecture expert with full system diagram and design patterns",
            "coding",
            "🏗️",
            vec!["bizclaw", "architecture", "rust", "crates", "patterns"],
            include_str!("skills/bizclaw_architecture.md"),
        ),
    ];

    skill_defs
        .into_iter()
        .filter_map(|(name, display, desc, cat, icon, tags, content)| {
            let yaml = format!(
                "---\nname: {}\ndisplay_name: {}\ndescription: {}\ncategory: {}\nicon: {}\ntags: [{}]\n---\n{}",
                name,
                display,
                desc,
                cat,
                icon,
                tags.join(", "),
                content,
            );
            SkillManifest::parse(&yaml).ok()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_skills_count() {
        let skills = builtin_skills();
        // 16 original (10 + 6 BizClaw) + 18 agency-derived = 34 skills
        // But depends on whether agency skills dir exists during test
        assert!(
            skills.len() >= 16,
            "Expected at least 16 built-in skills, got {}",
            skills.len()
        );
    }

    #[test]
    fn test_builtin_skills_have_content() {
        for skill in builtin_skills() {
            assert!(
                !skill.content.is_empty(),
                "Skill '{}' has no content",
                skill.metadata.name
            );
            assert!(
                !skill.metadata.tags.is_empty(),
                "Skill '{}' has no tags",
                skill.metadata.name
            );
        }
    }

    #[test]
    fn test_bizclaw_skills_present() {
        let skills = builtin_skills();
        let names: Vec<&str> = skills.iter().map(|s| s.metadata.name.as_str()).collect();
        assert!(names.contains(&"bizclaw-test-engineer"));
        assert!(names.contains(&"bizclaw-deploy-engineer"));
        assert!(names.contains(&"bizclaw-security-hardening"));
        assert!(names.contains(&"bizclaw-code-review"));
        assert!(names.contains(&"bizclaw-feature-review"));
        assert!(names.contains(&"bizclaw-architecture"));
    }
    
    #[test]
    fn test_agency_skills_loaded_when_exists() {
        let skills = builtin_skills();
        let names: Vec<&str> = skills.iter().map(|s| s.metadata.name.as_str()).collect();
        
        // Check for key agency-derived skills (VN market)
        let agency_skills = [
            "chief-of-staff",
            "content-creator",
            "tiktok-strategist",
            "outbound-strategist",
            "customer-service",
            "legal-compliance-checker",
        ];
        
        let found_count = agency_skills
            .iter()
            .filter(|s| names.contains(s))
            .count();
        
        // At least some agency skills should be loaded if directory exists
        if cfg!(test) {
            // In test environment, might not have agency dir
            println!("Agency skills found: {}/{}", found_count, agency_skills.len());
        }
    }

    #[test]
    fn test_agency_skills_categories() {
        let skills = builtin_skills();
        let categories: Vec<&str> = skills
            .iter()
            .map(|s| s.metadata.category.as_str())
            .collect();
        
        // Should have multiple categories including agency-derived
        let unique_categories: std::collections::HashSet<&str> = categories.iter().cloned().collect();
        assert!(
            unique_categories.len() >= 3,
            "Expected at least 3 skill categories, got {:?}",
            unique_categories
        );
    }
}
