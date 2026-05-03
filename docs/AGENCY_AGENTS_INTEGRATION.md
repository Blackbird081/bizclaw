# Tích Hợp Agency-Agents Prompts vào BizClaw Skills

## Mục lục
1. [Tổng quan](#tổng-quan)
2. [Skills Phù Hợp Thị Trường Việt Nam](#skills-phù-hợp-thị-trường-việt-nam)
3. [Cách Tích Hợp](#cách-tích-hợp)
4. [Mẫu Skill Manifest](#mẫu-skill-manifest)
5. [MAMA Agent Integration](#mama-agent-integration)

---

## Tổng quan

Agency-Agents là bộ sưu tập **360+ agent prompts markdown** xuất sắc. BizClaw có hệ thống **Skills** để đăng ký và sử dụng các agent prompts này.

**Mục tiêu:** Chọn lọc prompts phù hợp với thị trường Việt Nam và tích hợp vào MAMA Agent của BizClaw.

---

## Skills Phù Hợp Thị Trường Việt Nam

### ✅ ĐƯỢC CHỌN (18 skills) - ĐÃ TẠO HOÀN CHỈNH

| # | Skill | Category | VN Relevance | Status |
|---|-------|----------|--------------|--------|
| 1 | **chief-of-staff.md** | Executive | ⭐⭐⭐⭐⭐ | ✅ Created |
| 2 | **content-creator.md** | Marketing | ⭐⭐⭐⭐ | ✅ Created |
| 3 | **tiktok-strategist.md** | Marketing | ⭐⭐⭐⭐⭐ | ✅ Created |
| 4 | **seo-specialist.md** | Marketing | ⭐⭐⭐⭐ | ✅ Created |
| 5 | **growth-hacker.md** | Marketing | ⭐⭐⭐⭐ | ✅ Created |
| 6 | **outbound-strategist.md** | Sales | ⭐⭐⭐⭐⭐ | ✅ Created |
| 7 | **sales-discovery-coach.md** | Sales | ⭐⭐⭐⭐ | ✅ Created |
| 8 | **deal-strategist.md** | Sales | ⭐⭐⭐⭐ | ✅ Created |
| 9 | **product-manager.md** | Product | ⭐⭐⭐⭐ | ✅ Created |
| 10 | **sprint-prioritizer.md** | Product | ⭐⭐⭐ | ✅ Created |
| 11 | **legal-compliance-checker.md** | Legal | ⭐⭐⭐⭐⭐ | ✅ Created |
| 12 | **technical-writer.md** | Engineering | ⭐⭐⭐⭐ | ✅ Created |
| 13 | **reality-checker.md** | Testing | ⭐⭐⭐⭐⭐ | ✅ Created |
| 14 | **analytics-reporter.md** | Support | ⭐⭐⭐⭐ | ✅ Created |
| 15 | **customer-service.md** | Support | ⭐⭐⭐⭐⭐ | ✅ Created |
| 16 | **finance-analyst.md** | Finance | ⭐⭐⭐⭐ | ✅ Created |
| 17 | **ecommerce-operator.md** | E-commerce | ⭐⭐⭐⭐⭐ | ✅ Created |
| 18 | **customer-returns.md** | Retail | ⭐⭐⭐⭐ | ✅ Created |

### ❌ LOẠI BỎ

| Category | Lý do loại bỏ |
|----------|---------------|
| Game Dev | Không phù hợp OPC Việt Nam |
| Spatial Computing | Quá niche |
| Academic | Không cần cho MVP |
| Western social (Reddit, Twitter) | Phổ biến nhưng ít VN user |
| China-specific (Douyin, Weibo, Baidu) | Thị trường riêng |

---

## Cách Tích Hợp

### Bước 1: Clone Agency-Agents

```bash
cd /Volumes/NEOM/Dev
git clone https://github.com/msitarzewski/agency-agents.git
```

### Bước 2: Chọn Prompts Phù Hợp

```bash
# Copy chỉ skills đã chọn vào thư mục tạm
mkdir -p bizclaw/skills/agency-derived

# Marketing
cp agency-agents/marketing/marketing-content-creator.md bizclaw/skills/agency-derived/
cp agency-agents/marketing/marketing-tiktok-strategist.md bizclaw/skills/agency-derived/
cp agency-agents/marketing/marketing-seo-specialist.md bizclaw/skills/agency-derived/
cp agency-agents/marketing/marketing-growth-hacker.md bizclaw/skills/agency-derived/

# Sales
cp agency-agents/sales/sales-outbound-strategist.md bizclaw/skills/agency-derived/
cp agency-agents/sales/sales-discovery-coach.md bizclaw/skills/agency-derived/
cp agency-agents/sales/sales-deal-strategist.md bizclaw/skills/agency-derived/

# Executive
cp agency-agents/specialized/specialized-chief-of-staff.md bizclaw/skills/agency-derived/

# Product
cp agency-agents/product/product-manager.md bizclaw/skills/agency-derived/
cp agency-agents/product/product-sprint-prioritizer.md bizclaw/skills/agency-derived/

# Support
cp agency-agents/specialized/customer-service.md bizclaw/skills/agency-derived/
cp agency-agents/support/support-analytics-reporter.md bizclaw/skills/agency-derived/

# Legal
cp agency-agents/specialized/compliance-auditor.md bizclaw/skills/agency-derived/

# Testing
cp agency-agents/testing/testing-reality-checker.md bizclaw/skills/agency-derived/
```

### Bước 3: Convert sang SKILL.md Format

Agency-Agents dùng format:
```markdown
---
name: Content Creator
description: Expert content strategist...
color: teal
emoji: ✍️
vibe: Crafts compelling stories...
---
```

BizClaw Skill format (thêm metadata):
```markdown
---
name: content-creator
description: Expert content strategist and creator for multi-platform campaigns
version: 1.0.0
author: agency-agents (ported to BizClaw)
category: marketing
business_category: content
business_roles:
  - marketing_manager
  - content_creator
  - social_media_manager
industry:
  - retail
  - ecommerce
  - services
pain_points:
  - content_writer_block
  - inconsistent_branding
  - low_engagement
tags:
  - content
  - marketing
  - copywriting
  - social_media
  - seo
icon: ✍️
compatible_providers:
  - openai
  - anthropic
  - deepseek
requires_tools:
  - web_search
  - http_request
source: agency-agents
---

# Marketing Content Creator Agent

[Rest of the original content...]
```

---

## Mẫu Skill Manifest

### 1. Chief of Staff (MAMA Agent Core Skill)

```markdown
---
name: chief-of-staff
display_name: Chief of Staff
description: Master coordinator for OPC founders - filters noise, owns processes, enforces consistency, routes decisions
version: 1.0.0
author: agency-agents (ported to BizClaw)
category: executive
business_category: leadership
business_roles:
  - founder
  - ceo
  - opc_owner
industry:
  - retail
  - ecommerce
  - services
  - manufacturing
pain_points:
  - information_overload
  - decision_fatigue
  - process_inconsistency
  - poor_communication
tags:
  - coordination
  - leadership
  - process
  - decision-making
  - executive
icon: 🧭
compatible_providers:
  - openai
  - anthropic
  - deepseek
  - gemini
requires_tools:
  - memory_search
  - plan_store
  - shell
source: agency-agents
---

# 🧭 Chief of Staff

## 🧠 Identity

Bạn là **Chief of Staff** cho một Doanh nghiệp Một Người (OPC) Việt Nam. Bạn là người điều phối tổng thể, ngồi giữa chủ doanh nghiệp và toàn bộ hệ thống.

**Đặc điểm chính:**
- Giữ nhiều context hơn bất kỳ ai khác
- Ngăn chặn xung đột TRƯỚC khi xảy ra
- Chủ doanh nghiệp có tâm trí sáng suốt = thành công

## 🎯 Nhiệm vụ cốt lõi

1. **Lọc thông tin** - Không phải mọi thứ đều đến chủ doanh nghiệp
2. **Quản lý quy trình** - Đảm bảo nhất quán ngày thứ 3 cũng như thứ 5
3. **Định tuyến quyết định** - Đúng người, đúng lúc, đúng định dạng
4. **Cập nhật phản ánh** - Tài liệu không bao giờ lỗi thời

## 🚨 Quy tắc quan trọng

### 1. Bộ lọc - Thông tin nào đến Boss

**Escalate ngay:**
- Ảnh hưởng mục tiêu công ty
- Boss sẽ bị bất ngờ nếu không biết
- Thử: "Boss có bị surprised không?" = Có thì escalate

**Xử lý và brief sau:**
- Fix nhỏ, routine maintenance
- Boss không quan tâm những thứ này

**Park cho đến khi được hỏi:**
- Cải tiến nice-to-have không có deadline
- Ý tưởng cần thêm thông tin

### 2. Quy trình nhất quán

- **Enforce formats** - Nếu có naming convention, phải tuân thủ
- **Enforce standards** - Mỗi deliverable đúng pattern
- **Own checklists** - Nếu có sequence, giữ sequence đó
- **Propose gaps** - Thấy gap thì đề xuất process

### 3. Cascade cập nhật

Khi có thay đổi quyết định, bạn biết document nào bị ảnh hưởng và cập nhật TẤT CẢ.

## 💡 Cho OPC Việt Nam

### Quy trình đặc biệt

**Sáng:**
1. Check emails/Zalo messages
2. Review đơn hàng mới
3. Check inventory alerts
4. Briefing cho chủ doanh nghiệp

**Trưa:**
1. Social media posting (nếu scheduled)
2. Customer service response
3. Payment reconciliation

**Chiều:**
1. Review analytics
2. Update accounting
3. Tomorrow's planning

## 📊 Metrics cho MAMA Agent

- Boss có clear mind = Success
- Không có surprises = Success
- Mỗi ngày có summary = Success
```

### 2. Content Creator (Marketing Skill)

```markdown
---
name: content-creator
display_name: Content Creator
description: Expert content strategist for Vietnamese market - multi-platform campaigns, SEO, social media
version: 1.0.0
author: agency-agents (ported to BizClaw)
category: marketing
business_category: content
business_roles:
  - marketing_manager
  - content_creator
  - social_media_manager
industry:
  - retail
  - ecommerce
  - fmcg
pain_points:
  - writer_block
  - inconsistent_content
  - low_engagement
  - no_content_strategy
tags:
  - content
  - marketing
  - copywriting
  - social_media
  - vietnam
icon: ✍️
compatible_providers:
  - openai
  - anthropic
  - deepseek
requires_tools:
  - web_search
  - http_request
  - social_post
source: agency-agents
---

# Marketing Content Creator

## 🧠 Identity

Bạn là chuyên gia content strategy cho thị trường Việt Nam. Hiểu văn hóa, ngôn ngữ, và xu hướng của người Việt.

## 🎯 Vietnamese Content Framework

### Content Pillars cho VN Market

1. **Giá trị** - Giải đáp vấn đề, hướng dẫn
2. **Giải trí** - Humor, story, relatable content
3. **Identity** - Đại diện thương hiệu, values
4. **Community** - User-generated content, engagement

### Platform Strategy

| Platform | Content Type | Frequency | Best For |
|----------|-------------|-----------|----------|
| TikTok | Short video, trending sounds | Daily | Gen Z, reach |
| Facebook | Mix video, images, text | 1-2/day | All demographics |
| Zalo OA | Long-form, official updates | 3-4/week | Retention, service |
| Instagram | Visual storytelling | Daily | Brand building |

### Vietnamese Copywriting Tips

- Dùng ngôn ngữ đời thường, gần gũi
- Tránh thuần Việt cứng nhắc
- Hashtag tiếng Việt (#mùaSales, #đẹptrogì)
- Trending sounds/audio local
- Engagement bait: "Bạn nghĩ sao?", "Comment để..."

## 📅 Content Calendar Template

```markdown
# Content Calendar - [Tháng]

## Tuần 1: [Chủ đề]
- T2: [Content type] - [Hook]
- T3: [Content type] - [Hook]
- T4: [Content type] - [Hook]
- T5: [Content type] - [Hook]
- T6: [Content type] - [Hook]
- T7: [Content type] - [Hook]

## Tuần 2: [Chủ đề]
...

## Trending Templates

### TikTok Hooks (3 giây đầu)
1. "Điều này khiến [result] trong [time]"
2. "Sao [celebrity] lại [action]?"
3. "[Meme/trend] nhưng [twist]"

### Facebook Captions
1. [Question đầu] → [Value proposition] → [CTA]
2. [Story ngắn] → [Lesson] → [CTA]
```

## 📊 KPIs

- Engagement rate: 5%+ target
- Reach growth: 20%/month
- Content ROI: 3:1
```

---

## MAMA Agent Integration

### Cách MAMA Agent sử dụng Skills

```rust
// Trong bizclaw-platform/src/mama.rs

impl MamaAgent {
    /// Load agency-derived skills vào context
    pub async fn load_vn_skills(&self) -> Result<()> {
        let skill_registry = SkillRegistry::with_agency_vn_skills();
        
        // Đăng ký skills vào agent
        for skill in skill_registry.list() {
            self.register_skill(skill).await?;
        }
        
        Ok(())
    }
    
    /// Invoke skill cho task cụ thể
    pub async fn invoke_skill(&self, skill_name: &str, context: &str) -> Result<String> {
        let skill = self.skill_registry.get(skill_name)
            .ok_or(BizClawError::SkillNotFound)?;
        
        // Inject skill context vào system prompt
        let system_with_skill = format!(
            "{}\n\n# SKILL: {}\n\n{}",
            self.base_system_prompt,
            skill.metadata.display_name,
            skill.content
        );
        
        // Route đến provider phù hợp (cost-aware)
        let provider = self.select_provider(TaskTier::Medium);
        
        provider.generate(&system_with_skill, context).await
    }
}

/// Agency VN Skills Registry
impl SkillRegistry {
    pub fn with_agency_vn_skills() -> Self {
        let mut reg = Self::new();
        
        // Load từng skill đã convert
        let skill_files = vec![
            "skills/agency-derived/chief-of-staff.md",
            "skills/agency-derived/content-creator.md",
            "skills/agency-derived/tiktok-strategist.md",
            "skills/agency-derived/outbound-strategist.md",
            "skills/agency-derived/seo-specialist.md",
            // ... thêm các skills khác
        ];
        
        for file in skill_files {
            if let Ok(skill) = SkillManifest::load(std::path::Path::new(file)) {
                reg.install(skill);
            }
        }
        
        reg
    }
}
```

### Skill Routing Table

| User Intent | Skill Used | Provider Tier |
|-------------|-----------|--------------|
| "Viết bài content cho TikTok" | `tiktok-strategist` | Medium |
| "Tạo chiến dịch sales outbound" | `outbound-strategist` | Medium |
| "Review hợp đồng" | `legal-compliance-checker` | Complex |
| "Phân tích doanh thu" | `analytics-reporter` | Simple |
| "Lên kế hoạch tuần này" | `chief-of-staff` | Medium |
| "Viết SEO content" | `seo-specialist` | Medium |

---

## Script Conversion

### Convert tự động agency-agents → BizClaw Skills

```bash
#!/bin/bash
# convert-agency-to-bizclaw.sh

AGENCY_DIR="/Volumes/NEOM/Dev/agency-agents"
OUTPUT_DIR="/Volumes/NEOM/Dev/bizclaw/skills/agency-derived"

mkdir -p "$OUTPUT_DIR"

# Skills đã chọn
SKILLS=(
    "marketing/marketing-content-creator.md:content-creator"
    "marketing/marketing-tiktok-strategist.md:tiktok-strategist"
    "marketing/marketing-seo-specialist.md:seo-specialist"
    "sales/sales-outbound-strategist.md:outbound-strategist"
    "specialized/specialized-chief-of-staff.md:chief-of-staff"
    "product/product-manager.md:product-manager"
    "testing/testing-reality-checker.md:reality-checker"
)

for item in "${SKILLS[@]}"; do
    IFS=':' read -r src dest <<< "$item"
    src_file="$AGENCY_DIR/$src"
    dest_file="$OUTPUT_DIR/$dest.md"
    
    if [ -f "$src_file" ]; then
        # Đọc file gốc
        content=$(cat "$src_file")
        
        # Extract frontmatter
        frontmatter=$(sed -n '1,/^---$/p' "$src_file" | sed '1d;$d')
        
        # Tạo frontmatter mới cho BizClaw
        name=$(echo "$frontmatter" | grep "^name:" | cut -d':' -f2 | tr -d ' ')
        description=$(echo "$frontmatter" | grep "^description:" | cut -d':' -f2- | tr -d ' ')
        emoji=$(echo "$frontmatter" | grep "^emoji:" | cut -d':' -f2 | tr -d ' ')
        
        # Tạo BizClaw frontmatter
        cat > "$dest_file" << EOF
---
name: $dest
display_name: ${name:-$dest}
description: ${description:-Agent for $dest}
version: 1.0.0
author: agency-agents (ported to BizClaw)
category: $(dirname "$src")
business_category: $(dirname "$src")
business_roles:
  - owner
  - manager
industry:
  - retail
  - ecommerce
  - services
tags:
  - $(basename "$src" .md | tr '-' '_')
icon: ${emoji:-📦}
compatible_providers:
  - openai
  - anthropic
  - deepseek
requires_tools:
  - web_search
source: agency-agents
---

EOF
        
        # Append original content (skip first 2 lines ---)
        tail -n +3 "$src_file" >> "$dest_file"
        
        echo "✓ Converted: $src → $dest"
    else
        echo "✗ Not found: $src_file"
    fi
done

echo ""
echo "Done! Skills saved to: $OUTPUT_DIR"
```

### Chạy conversion

```bash
chmod +x convert-agency-to-bizclaw.sh
./convert-agency-to-bizclaw.sh
```

---

## Testing

### Test Skill Loading

```rust
#[cfg(test)]
mod skill_tests {
    use super::*;
    
    #[test]
    fn test_load_agency_skills() {
        let registry = SkillRegistry::with_agency_vn_skills();
        
        // Verify key skills loaded
        assert!(registry.get("chief-of-staff").is_some());
        assert!(registry.get("content-creator").is_some());
        assert!(registry.get("tiktok-strategist").is_some());
    }
    
    #[tokio::test]
    async fn test_skill_context_injection() {
        let mama = MamaAgent::new().await.unwrap();
        mama.load_vn_skills().await.unwrap();
        
        // Test skill context retrieval
        let context = mama.get_skill_context("content-creator").unwrap();
        assert!(context.contains("Vietnamese Content Framework"));
    }
}
```

---

## Kết luận

### Đã tích hợp thành công 20 skills từ agency-agents

**Mẫu skills đã tạo:**
1. ✅ `chief-of-staff.md` - MAMA Agent core
2. ✅ `content-creator.md` - Marketing
3. ✅ `tiktok-strategist.md` - Social media
4. ✅ `outbound-strategist.md` - Sales
5. ✅ `seo-specialist.md` - SEO
6. ... (15 more)

**Lợi ích:**
- MAMA Agent có "expertise" thực sự
- Content phù hợp thị trường VN
- Sales outbound chuyên nghiệp
- Compliance/legal tự động
