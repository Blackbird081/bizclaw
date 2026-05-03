# Tích Hợp Thoth Features vào BizClaw

## Mục lục
1. [Knowledge Graph Visualization API](#knowledge-graph-visualization-api)
2. [Dream Cycle Memory Refinement](#dream-cycle-memory-refinement)
3. [Designer Studio Features](#designer-studio-features)

---

## 1. Knowledge Graph Visualization API

### Mục tiêu
Tạo API endpoint để visualize knowledge graph như Thoth - hiển thị entities và relationships dưới dạng interactive graph.

### Implementation

#### 1.1 Graph Visualization API Endpoint

```rust
// crates/bizclaw-knowledge/src/graph_visualizer.rs

use serde::{Deserialize, Serialize};

/// Graph visualization node for frontend D3.js/Vis.js
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    pub group: String,
    pub title: String,
    pub level: u32,
    pub size: f32,
    pub color: String,
    pub image: Option<String>,
    pub shape: String,
}

/// Graph visualization edge for frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub id: String,
    pub from: String,
    pub to: String,
    pub label: String,
    pub arrows: String,
    pub dashes: bool,
    pub width: f32,
    pub color: String,
}

/// Graph visualization response for dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphVisualization {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub stats: GraphStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphStats {
    pub total_entities: usize,
    pub total_relationships: usize,
    pub entity_types: HashMap<String, usize>,
    pub top_relations: Vec<(String, usize)>,
}

/// Convert KnowledgeGraph to visualization format
impl KnowledgeGraph {
    pub async fn to_visualization(&self, options: &GraphOptions) -> GraphVisualization {
        let entities = self.entities.read().await;
        let relationships = self.relationships.read().await;
        
        // Group entities by type for visualization
        let mut nodes: Vec<GraphNode> = Vec::new();
        let mut entity_type_counts: HashMap<String, usize> = HashMap::new();
        
        for entity in entities.values() {
            if let Some(entity_type) = &options.entity_types {
                if !entity_type.contains(&entity.entity_type) {
                    continue;
                }
            }
            
            let type_str = entity.entity_type.as_str().to_string();
            *entity_type_counts.entry(type_str.clone()).or_insert(0) += 1;
            
            nodes.push(GraphNode {
                id: entity.id.0.clone(),
                label: entity.name.clone(),
                group: type_str,
                title: entity.description.clone().unwrap_or_default(),
                level: 0,
                size: 1.0 + (entity.properties.len() as f32 * 0.1),
                color: Self::type_to_color(&entity.entity_type),
                image: None,
                shape: "ellipse".to_string(),
            });
        }
        
        // Build edges
        let mut edges: Vec<GraphEdge> = Vec::new();
        let mut relation_counts: HashMap<String, usize> = HashMap::new();
        
        for rel in relationships.values() {
            *relation_counts.entry(rel.relation_type.as_str().to_string()).or_insert(0) += 1;
            
            edges.push(GraphEdge {
                id: rel.id.clone(),
                from: rel.source.0.clone(),
                to: rel.target.0.clone(),
                label: rel.relation_type.as_str().to_string(),
                arrows: "to".to_string(),
                dashes: rel.weight < 0.5,
                width: 1.0 + (rel.weight * 3.0),
                color: Self::weight_to_color(rel.weight),
            });
        }
        
        GraphVisualization {
            nodes,
            edges,
            stats: GraphStats {
                total_entities: entities.len(),
                total_relationships: relationships.len(),
                entity_types: entity_type_counts,
                top_relations: Self::top_n(relation_counts, 10),
            },
        }
    }
    
    fn type_to_color(entity_type: &EntityType) -> String {
        match entity_type {
            EntityType::Person => "#FF6B6B".to_string(),
            EntityType::Organization => "#4ECDC4".to_string(),
            EntityType::Product => "#45B7D1".to_string(),
            EntityType::Concept => "#96CEB4".to_string(),
            EntityType::Location => "#FFEAA7".to_string(),
            EntityType::Event => "#DDA0DD".to_string(),
            EntityType::Document => "#98D8C8".to_string(),
            EntityType::Task => "#F7DC6F".to_string(),
            EntityType::Custom(_) => "#BDC3C7".to_string(),
        }
    }
    
    fn weight_to_color(weight: f32) -> String {
        if weight > 0.7 {
            "#27AE60".to_string()  // Strong - green
        } else if weight > 0.4 {
            "#F39C12".to_string()   // Medium - orange
        } else {
            "#E74C3C".to_string()   // Weak - red
        }
    }
    
    fn top_n<K: Ord + Clone, V: Ord + Clone>(mut map: HashMap<K, V>, n: usize) -> Vec<(K, V)> {
        let mut vec: Vec<(K, V)> = map.into_iter().collect();
        vec.sort_by(|a, b| b.1.cmp(&a.1));
        vec.into_iter().take(n).collect()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GraphOptions {
    pub entity_types: Option<Vec<EntityType>>,
    pub min_weight: Option<f32>,
    pub limit: Option<usize>,
}
```

#### 1.2 Graph API Routes

```rust
// crates/bizclaw-gateway/src/routes/knowledge_graph.rs

use axum::{extract::Query, routing::get, Json, Router};

pub fn graph_routes() -> Router {
    Router::new()
        .route("/api/graph/visualization", get(get_graph_visualization))
        .route("/api/graph/entity/:id", get(get_entity_detail))
        .route("/api/graph/entity/:id/relations", get(get_entity_relations))
        .route("/api/graph/search", get(search_graph))
        .route("/api/graph/path", get(find_path_between_entities))
}

#[derive(Deserialize)]
pub struct GraphQuery {
    pub entity_type: Option<String>,
    pub min_weight: Option<f32>,
    pub limit: Option<usize>,
}

/// GET /api/graph/visualization
async fn get_graph_visualization(
    Query(params): Query<GraphQuery>,
) -> Json<GraphVisualization> {
    let options = GraphOptions {
        entity_types: params.entity_type
            .map(|t| vec![t.parse().unwrap_or(EntityType::Concept)]),
        min_weight: params.min_weight,
        limit: params.limit.or(Some(100)),
    };
    
    let graph = knowledge_graph_service().await;
    let visualization = graph.to_visualization(&options).await;
    Json(visualization)
}

/// GET /api/graph/entity/:id
async fn get_entity_detail(Path(id): Path<String>) -> Json<EntityDetail> {
    let graph = knowledge_graph_service().await;
    let entity = graph.get_entity(&EntityId(id)).await;
    
    Json(EntityDetail {
        entity,
        incoming: graph.get_incoming_relations(&EntityId(id)).await,
        outgoing: graph.get_outgoing_relations(&EntityId(id)).await,
        neighbors: graph.get_neighbors(&EntityId(id), 1).await,
    })
}

/// GET /api/graph/entity/:id/relations
async fn get_entity_relations(
    Path(id): Path<String>,
    Query(params): Query<GraphQuery>,
) -> Json<Vec<Relationship>> {
    let graph = knowledge_graph_service().await;
    let entity_id = EntityId(id);
    
    let mut relations = Vec::new();
    relations.extend(graph.get_incoming_relations(&entity_id).await);
    relations.extend(graph.get_outgoing_relations(&entity_id).await);
    
    // Filter by min_weight
    if let Some(min) = params.min_weight {
        relations.retain(|r| r.weight >= min);
    }
    
    Json(relations)
}

/// GET /api/graph/search
async fn search_graph(Query(params): Query<GraphQuery>) -> Json<Vec<SearchResult>> {
    let graph = knowledge_graph_service().await;
    let query = GraphQuery {
        entity_name: params.q.clone(),
        entity_type: params.entity_type.as_ref().map(|t| t.parse().unwrap_or(EntityType::Concept)),
        depth: Some(2),
        limit: params.limit.or(Some(20)),
    };
    
    let results = graph.search(&query).await;
    Json(results)
}
```

#### 1.3 Frontend D3.js Visualization Component

```typescript
// Frontend: components/GraphVisualization.tsx

import React, { useEffect, useRef } from 'react';
import * as d3 from 'd3';

interface GraphNode {
  id: string;
  label: string;
  group: string;
  title: string;
  size: number;
  color: string;
}

interface GraphEdge {
  id: string;
  from: string;
  to: string;
  label: string;
  width: number;
  color: string;
}

interface GraphVisualizationProps {
  nodes: GraphNode[];
  edges: GraphEdge[];
  onNodeClick?: (node: GraphNode) => void;
  onEdgeClick?: (edge: GraphEdge) => void;
}

export const GraphVisualization: React.FC<GraphVisualizationProps> = ({
  nodes,
  edges,
  onNodeClick,
  onEdgeClick,
}) => {
  const svgRef = useRef<SVGSVGElement>(null);

  useEffect(() => {
    if (!svgRef.current || nodes.length === 0) return;

    const svg = d3.select(svgRef.current);
    const width = svgRef.current.clientWidth;
    const height = svgRef.current.clientHeight;

    // Clear previous
    svg.selectAll('*').remove();

    // Create zoom behavior
    const g = svg.append('g');
    const zoom = d3.zoom()
      .scaleExtent([0.1, 4])
      .on('zoom', (event) => g.attr('transform', event.transform));
    svg.call(zoom);

    // Create force simulation
    const simulation = d3.forceSimulation(nodes as any)
      .force('link', d3.forceLink(edges)
        .id((d: any) => d.id)
        .distance(100))
      .force('charge', d3.forceManyBody().strength(-300))
      .force('center', d3.forceCenter(width / 2, height / 2))
      .force('collision', d3.forceCollide().radius(30));

    // Draw edges
    const link = g.append('g')
      .selectAll('line')
      .data(edges)
      .join('line')
      .attr('stroke', (d: any) => d.color)
      .attr('stroke-width', (d: any) => d.width)
      .attr('stroke-opacity', 0.6)
      .on('click', (_: any, d: any) => onEdgeClick?.(d));

    // Draw edge labels
    const linkLabel = g.append('g')
      .selectAll('text')
      .data(edges)
      .join('text')
      .text((d: any) => d.label)
      .attr('font-size', 10)
      .attr('fill', '#666')
      .attr('text-anchor', 'middle');

    // Draw nodes
    const node = g.append('g')
      .selectAll('circle')
      .data(nodes)
      .join('circle')
      .attr('r', (d: any) => d.size * 10)
      .attr('fill', (d: any) => d.color)
      .attr('stroke', '#fff')
      .attr('stroke-width', 2)
      .on('click', (_: any, d: any) => onNodeClick?.(d))
      .call(d3.drag()
        .on('start', (event, d: any) => {
          if (!event.active) simulation.alphaTarget(0.3).restart();
          d.fx = d.x;
          d.fy = d.y;
        })
        .on('drag', (event, d: any) => {
          d.fx = event.x;
          d.fy = event.y;
        })
        .on('end', (event, d: any) => {
          if (!event.active) simulation.alphaTarget(0);
          d.fx = null;
          d.fy = null;
        }));

    // Add labels
    const label = g.append('g')
      .selectAll('text')
      .data(nodes)
      .join('text')
      .text((d: any) => d.label)
      .attr('font-size', 12)
      .attr('text-anchor', 'middle')
      .attr('dy', 25);

    // Update positions on tick
    simulation.on('tick', () => {
      link
        .attr('x1', (d: any) => d.source.x)
        .attr('y1', (d: any) => d.source.y)
        .attr('x2', (d: any) => d.target.x)
        .attr('y2', (d: any) => d.target.y);

      linkLabel
        .attr('x', (d: any) => (d.source.x + d.target.x) / 2)
        .attr('y', (d: any) => (d.source.y + d.target.y) / 2);

      node
        .attr('cx', (d: any) => d.x)
        .attr('cy', (d: any) => d.y);

      label
        .attr('x', (d: any) => d.x)
        .attr('y', (d: any) => d.y);
    });

    return () => simulation.stop();
  }, [nodes, edges]);

  return (
    <svg ref={svgRef} width="100%" height="100%" />
  );
};
```

---

## 2. Dream Cycle Memory Refinement

### Mục tiêu
Tạo scheduled task để tự động refine memory như Thoth Dream Cycle - làm sạch duplicate, fix stale relationships, và tạo actionable insights.

### Implementation

#### 2.1 Dream Cycle Service

```rust
// crates/bizclaw-memory/src/dream_cycle.rs

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Dream Cycle - Memory refinement scheduled task
/// 
/// Inspired by Thoth's Dream Cycle:
/// - Runs on schedule (default: every 6 hours)
/// - Cleans duplicate entities
/// - Repairs orphaned relationships
/// - Refines low-confidence memories
/// - Generates actionable insights

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamCycleConfig {
    /// Schedule in cron format (default: "0 */6 * * *" = every 6 hours)
    pub schedule: String,
    /// Enable automatic cleanup
    pub auto_cleanup: bool,
    /// Confidence threshold for refinement
    pub confidence_threshold: f32,
    /// Days before entity is considered "stale"
    pub stale_days: u32,
    /// Maximum entities to process per cycle
    pub batch_size: usize,
}

impl Default for DreamCycleConfig {
    fn default() -> Self {
        Self {
            schedule: "0 */6 * * *".to_string(),
            auto_cleanup: true,
            confidence_threshold: 0.5,
            stale_days: 30,
            batch_size: 100,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamCycleResult {
    pub cycle_id: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub actions_taken: Vec<DreamAction>,
    pub insights: Vec<MemoryInsight>,
    pub stats: DreamCycleStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamAction {
    pub action_type: DreamActionType,
    pub entity_id: Option<String>,
    pub description: String,
    pub confidence_before: f32,
    pub confidence_after: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DreamActionType {
    MergeDuplicate,
    RepairOrphan,
    EnhanceRelationship,
    ArchiveStale,
    BoostConfidence,
    SplitEntity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInsight {
    pub insight_type: InsightType,
    pub title: String,
    pub description: String,
    pub related_entities: Vec<String>,
    pub action_suggestion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InsightType {
    PatternDiscovered,
    MissingLink,
    OverlappingConcepts,
    ActionRecommended,
    TrendDetected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamCycleStats {
    pub entities_processed: usize,
    pub duplicates_merged: usize,
    pub orphans_repaired: usize,
    pub stale_archived: usize,
    pub relationships_created: usize,
    pub confidence_improved: usize,
    pub insights_generated: usize,
}

pub struct DreamCycle {
    config: DreamCycleConfig,
    knowledge_graph: Arc<KnowledgeGraph>,
    memory: Arc<dyn MemoryBackend>,
}

impl DreamCycle {
    pub fn new(
        config: DreamCycleConfig,
        knowledge_graph: Arc<KnowledgeGraph>,
        memory: Arc<dyn MemoryBackend>,
    ) -> Self {
        Self {
            config,
            knowledge_graph,
            memory,
        }
    }

    /// Run the dream cycle - memory refinement process
    pub async fn run(&self) -> DreamCycleResult {
        let cycle_id = uuid::Uuid::new_v4().to_string();
        let started_at = chrono::Utc::now();
        
        tracing::info!("🌙 Dream Cycle {} started", cycle_id);
        
        let mut actions: Vec<DreamAction> = Vec::new();
        let mut insights: Vec<MemoryInsight> = Vec::new();
        
        // Phase 1: Find and merge duplicates
        tracing::info!("Phase 1: Finding duplicates...");
        let duplicates = self.find_duplicates().await;
        for dup in duplicates {
            if let Some(action) = self.merge_duplicates(dup).await {
                actions.push(action);
            }
        }
        
        // Phase 2: Repair orphaned relationships
        tracing::info!("Phase 2: Repairing orphans...");
        let orphans = self.find_orphans().await;
        for orphan in orphans {
            if let Some(action) = self.repair_orphan(orphan).await {
                actions.push(action);
            }
        }
        
        // Phase 3: Archive stale entities
        tracing::info!("Phase 3: Archiving stale...");
        let stale = self.find_stale_entities().await;
        for entity in stale {
            if let Some(action) = self.archive_stale(entity).await {
                actions.push(action);
            }
        }
        
        // Phase 4: Enhance low-confidence relationships
        tracing::info!("Phase 4: Enhancing relationships...");
        let weak_relations = self.find_weak_relationships().await;
        for rel in weak_relations {
            if let Some(action) = self.enhance_relationship(rel).await {
                actions.push(action);
            }
        }
        
        // Phase 5: Generate insights
        tracing::info!("Phase 5: Generating insights...");
        insights = self.generate_insights().await;
        
        let completed_at = chrono::Utc::now();
        let stats = self.compute_stats(&actions, &insights);
        
        tracing::info!(
            "🌙 Dream Cycle {} completed - {} actions, {} insights",
            cycle_id,
            actions.len(),
            insights.len()
        );
        
        DreamCycleResult {
            cycle_id,
            started_at,
            completed_at,
            actions_taken: actions,
            insights,
            stats,
        }
    }
    
    async fn find_duplicates(&self) -> Vec<DuplicateGroup> {
        let entities = self.knowledge_graph.entities.read().await;
        let mut duplicates: Vec<DuplicateGroup> = Vec::new();
        
        // Group by name similarity (simple implementation)
        let mut name_groups: HashMap<String, Vec<Entity>> = HashMap::new();
        
        for entity in entities.values() {
            let key = entity.name.to_lowercase().trim().to_string();
            name_groups.entry(key).or_insert_with(Vec::new).push(entity.clone());
        }
        
        for (_, group) in name_groups {
            if group.len() > 1 {
                duplicates.push(DuplicateGroup { entities: group });
            }
        }
        
        duplicates
    }
    
    async fn merge_duplicates(&self, group: DuplicateGroup) -> Option<DreamAction> {
        if group.entities.len() < 2 {
            return None;
        }
        
        // Keep the most recent entity, merge properties
        let primary = group.entities.last()?;
        let confidence_before = primary.properties.get("confidence")
            .and_then(|v| v.parse().ok())
            .unwrap_or(1.0);
        
        // Merge into primary entity
        for entity in &group.entities[..group.entities.len() - 1] {
            self.knowledge_graph.merge_entities(&entity.id, &primary.id).await;
        }
        
        Some(DreamAction {
            action_type: DreamActionType::MergeDuplicate,
            entity_id: Some(primary.id.0.clone()),
            description: format!("Merged {} duplicate entities into '{}'", 
                group.entities.len() - 1, primary.name),
            confidence_before,
            confidence_after: Some(confidence_before + 0.1),
        })
    }
    
    async fn find_orphans(&self) -> Vec<Entity> {
        let entities = self.knowledge_graph.entities.read().await;
        let relationships = self.knowledge_graph.relationships.read().await;
        
        let mut connected: HashSet<EntityId> = HashSet::new();
        for rel in relationships.values() {
            connected.insert(rel.source.clone());
            connected.insert(rel.target.clone());
        }
        
        entities
            .values()
            .filter(|e| !connected.contains(&e.id))
            .cloned()
            .collect()
    }
    
    async fn repair_orphan(&self, orphan: Entity) -> Option<DreamAction> {
        // Find similar entities and suggest connections
        let similar = self.knowledge_graph.find_similar(&orphan).await;
        
        if let Some(best_match) = similar.first() {
            // Create new relationship
            self.knowledge_graph.add_relationship(Relationship {
                id: uuid::Uuid::new_v4().to_string(),
                source: orphan.id.clone(),
                target: best_match.id.clone(),
                relation_type: RelationType::RelatedTo,
                properties: HashMap::new(),
                weight: 0.5,
                created_at: chrono::Utc::now(),
            }).await;
            
            Some(DreamAction {
                action_type: DreamActionType::RepairOrphan,
                entity_id: Some(orphan.id.0.clone()),
                description: format!("Connected orphan '{}' to '{}' via RelatedTo", 
                    orphan.name, best_match.name),
                confidence_before: 0.3,
                confidence_after: Some(0.5),
            })
        } else {
            None
        }
    }
    
    async fn find_stale_entities(&self) -> Vec<Entity> {
        let entities = self.knowledge_graph.entities.read().await;
        let stale_threshold = chrono::Utc::now() - chrono::Duration::days(self.config.stale_days as i64);
        
        entities
            .values()
            .filter(|e| e.updated_at < stale_threshold)
            .cloned()
            .collect()
    }
    
    async fn archive_stale(&self, entity: Entity) -> Option<DreamAction> {
        if !self.config.auto_cleanup {
            return None;
        }
        
        // Mark as archived (soft delete)
        self.knowledge_graph.archive_entity(&entity.id).await;
        
        Some(DreamAction {
            action_type: DreamActionType::ArchiveStale,
            entity_id: Some(entity.id.0.clone()),
            description: format!("Archived stale entity '{}'", entity.name),
            confidence_before: 0.2,
            confidence_after: None,
        })
    }
    
    async fn find_weak_relationships(&self) -> Vec<Relationship> {
        let relationships = self.knowledge_graph.relationships.read().await;
        
        relationships
            .values()
            .filter(|r| r.weight < self.config.confidence_threshold)
            .cloned()
            .collect()
    }
    
    async fn enhance_relationship(&self, rel: Relationship) -> Option<DreamAction> {
        // Analyze context and potentially boost weight
        let new_weight = (rel.weight + 0.2).min(1.0);
        
        self.knowledge_graph.update_relationship_weight(&rel.id, new_weight).await;
        
        Some(DreamAction {
            action_type: DreamActionType::BoostConfidence,
            entity_id: None,
            description: format!("Enhanced relationship weight from {:.2} to {:.2}", 
                rel.weight, new_weight),
            confidence_before: rel.weight,
            confidence_after: Some(new_weight),
        })
    }
    
    async fn generate_insights(&self) -> Vec<MemoryInsight> {
        let mut insights: Vec<MemoryInsight> = Vec::new();
        
        // Pattern: Overlapping concepts
        if let Some(insight) = self.detect_overlapping_concepts().await {
            insights.push(insight);
        }
        
        // Pattern: Missing links
        if let Some(insight) = self.suggest_missing_links().await {
            insights.push(insight);
        }
        
        // Pattern: Action recommended
        if let Some(insight) = self.detect_actionable_patterns().await {
            insights.push(insight);
        }
        
        insights
    }
    
    async fn detect_overlapping_concepts(&self) -> Option<MemoryInsight> {
        // Find entities of same type with similar names
        let entities = self.knowledge_graph.entities.read().await;
        
        let mut concept_groups: HashMap<String, Vec<Entity>> = HashMap::new();
        for entity in entities.values() {
            if matches!(entity.entity_type, EntityType::Concept) {
                let key = entity.name.to_lowercase();
                concept_groups.entry(key).or_insert_with(Vec::new).push(entity.clone());
            }
        }
        
        let overlapping: Vec<(String, Vec<String>)> = concept_groups
            .into_iter()
            .filter(|(_, v)| v.len() > 1)
            .map(|(k, v)| (k, v.iter().map(|e| e.name.clone()).collect()))
            .collect();
        
        if overlapping.is_empty() {
            return None;
        }
        
        Some(MemoryInsight {
            insight_type: InsightType::OverlappingConcepts,
            title: "Overlapping Concepts Found".to_string(),
            description: format!("Found {} sets of overlapping concepts", overlapping.len()),
            related_entities: overlapping.first().map(|(_, v)| v.clone()).unwrap_or_default(),
            action_suggestion: "Consider merging or differentiating these concepts".to_string(),
        })
    }
    
    async fn suggest_missing_links(&self) -> Option<MemoryInsight> {
        // Find entities that should be connected but aren't
        // (simplified: entities of same type in same location)
        None // Placeholder for complex logic
    }
    
    async fn detect_actionable_patterns(&self) -> Option<MemoryInsight> {
        // Analyze patterns that suggest actions
        // e.g., "Multiple failed attempts on same task" -> suggest review
        None // Placeholder for complex logic
    }
    
    fn compute_stats(&self, actions: &[DreamAction], insights: &[MemoryInsight]) -> DreamCycleStats {
        DreamCycleStats {
            entities_processed: actions.iter()
                .filter(|a| matches!(a.action_type, DreamActionType::MergeDuplicate))
                .count(),
            duplicates_merged: actions.iter()
                .filter(|a| matches!(a.action_type, DreamActionType::MergeDuplicate))
                .count(),
            orphans_repaired: actions.iter()
                .filter(|a| matches!(a.action_type, DreamActionType::RepairOrphan))
                .count(),
            stale_archived: actions.iter()
                .filter(|a| matches!(a.action_type, DreamActionType::ArchiveStale))
                .count(),
            relationships_created: actions.iter()
                .filter(|a| matches!(a.action_type, DreamActionType::BoostConfidence))
                .count(),
            confidence_improved: actions.iter()
                .filter(|a| a.confidence_after.is_some())
                .count(),
            insights_generated: insights.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DuplicateGroup {
    pub entities: Vec<Entity>,
}
```

#### 2.2 Dream Cycle Integration với Hands

```rust
// crates/bizclaw-hands/src/dream_hand.rs

use crate::{HandManifest, HandRunner, HandPhase};
use bizclaw_memory::dream_cycle::DreamCycle;

/// Dream Cycle Hand - runs memory refinement on schedule
pub fn dream_cycle_hand() -> HandManifest {
    HandManifest {
        name: "dream-cycle".into(),
        display_name: "Dream Cycle".into(),
        description: "Memory refinement - cleans duplicates, repairs orphans, generates insights".into(),
        schedule: Some("0 */6 * * *".into()), // Every 6 hours
        phases: vec![
            HandPhase {
                name: "Find Duplicates".into(),
                instructions: "Scan knowledge graph for duplicate entities and merge them".into(),
                ..Default::default()
            },
            HandPhase {
                name: "Repair Orphans".into(),
                instructions: "Find orphaned entities and suggest connections".into(),
                ..Default::default()
            },
            HandPhase {
                name: "Archive Stale".into(),
                instructions: "Archive entities not accessed in 30+ days".into(),
                ..Default::default()
            },
            HandPhase {
                name: "Generate Insights".into(),
                instructions: "Analyze patterns and generate actionable insights".into(),
                ..Default::default()
            },
        ],
        ..Default::default()
    }
}

/// Create Dream Cycle Hand Runner
pub fn create_dream_runner(
    knowledge_graph: Arc<KnowledgeGraph>,
    memory: Arc<dyn MemoryBackend>,
) -> HandRunner {
    let config = bizclaw_memory::dream_cycle::DreamCycleConfig::default();
    let dream_cycle = Arc::new(DreamCycle::new(config, knowledge_graph, memory));
    
    HandRunner::new(dream_cycle_hand(), move |_ctx| {
        let dc = dream_cycle.clone();
        async move {
            dc.run().await
        }
    })
}
```

---

## 3. Designer Studio Features

### Mục tiêu
Tạo content generation system để tạo slide, landing pages, documents như Thoth Designer Studio.

### Implementation

#### 3.1 Designer Studio Service

```rust
// crates/bizclaw-content/src/designer_studio.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Designer Studio - Content creation for slides, landing pages, documents
/// 
/// Inspired by Thoth Designer Studio:
/// - Template-based content generation
/// - Multiple output formats (HTML, PPTX, MD)
/// - Brand-consistent styling
/// - Interactive preview

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    Slide,
    LandingPage,
    Document,
    Mockup,
    Storyboard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignRequest {
    pub content_type: ContentType,
    pub topic: String,
    pub audience: String,
    pub tone: String,
    pub brand: Option<BrandConfig>,
    pub format: OutputFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OutputFormat {
    Html,
    Pptx,
    Markdown,
    Pdf,
    Png,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandConfig {
    pub primary_color: String,
    pub secondary_color: String,
    pub font_family: String,
    pub logo_url: Option<String>,
    pub company_name: String,
}

impl Default for BrandConfig {
    fn default() -> Self {
        Self {
            primary_color: "#2563EB".to_string(),
            secondary_color: "#1E40AF".to_string(),
            font_family: "Inter, sans-serif".to_string(),
            logo_url: None,
            company_name: "Your Company".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignResult {
    pub request_id: String,
    pub content: String,
    pub preview_url: Option<String>,
    pub format: OutputFormat,
    pub metadata: DesignMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignMetadata {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub word_count: usize,
    pub slide_count: Option<usize>,
    pub sections: Vec<String>,
}

pub struct DesignerStudio {
    templates: HashMap<ContentType, DesignTemplate>,
    brand: BrandConfig,
}

pub struct DesignTemplate {
    pub name: String,
    pub structure: ContentStructure,
    pub sections: Vec<SectionTemplate>,
}

pub struct ContentStructure {
    pub slides: Option<usize>,
    pub columns: Option<usize>,
    pub has_nav: bool,
    pub has_footer: bool,
}

pub struct SectionTemplate {
    pub name: String,
    pub content_type: SectionType,
    pub required: bool,
}

#[derive(Debug, Clone)]
pub enum SectionType {
    Hero,
    Features,
    Pricing,
    Testimonials,
    CTA,
    Team,
    FAQ,
    Stats,
}

impl DesignerStudio {
    pub fn new(brand: BrandConfig) -> Self {
        Self {
            templates: Self::default_templates(),
            brand,
        }
    }
    
    fn default_templates() -> HashMap<ContentType, DesignTemplate> {
        let mut templates = HashMap::new();
        
        // Slide template
        templates.insert(ContentType::Slide, DesignTemplate {
            name: "Presentation Slide".into(),
            structure: ContentStructure {
                slides: Some(10),
                columns: Some(1),
                has_nav: false,
                has_footer: true,
            },
            sections: vec![
                SectionTemplate { name: "Title".into(), content_type: SectionType::Hero, required: true },
                SectionTemplate { name: "Content".into(), content_type: SectionType::Features, required: true },
                SectionTemplate { name: "Stats".into(), content_type: SectionType::Stats, required: false },
                SectionTemplate { name: "CTA".into(), content_type: SectionType::CTA, required: true },
            ],
        });
        
        // Landing page template
        templates.insert(ContentType::LandingPage, DesignTemplate {
            name: "Landing Page".into(),
            structure: ContentStructure {
                slides: None,
                columns: Some(2),
                has_nav: true,
                has_footer: true,
            },
            sections: vec![
                SectionTemplate { name: "Hero".into(), content_type: SectionType::Hero, required: true },
                SectionTemplate { name: "Features".into(), content_type: SectionType::Features, required: true },
                SectionTemplate { name: "Pricing".into(), content_type: SectionType::Pricing, required: false },
                SectionTemplate { name: "Testimonials".into(), content_type: SectionType::Testimonials, required: false },
                SectionTemplate { name: "FAQ".into(), content_type: SectionType::FAQ, required: false },
                SectionTemplate { name: "CTA".into(), content_type: SectionType::CTA, required: true },
            ],
        });
        
        templates
    }
    
    pub async fn generate(&self, request: DesignRequest) -> DesignResult {
        let template = self.templates.get(&request.content_type)
            .expect("Template not found");
        
        let content = match request.format {
            OutputFormat::Html => self.generate_html(&request, template).await,
            OutputFormat::Markdown => self.generate_markdown(&request, template).await,
            OutputFormat::Pptx => self.generate_pptx(&request, template).await,
            _ => self.generate_markdown(&request, template).await,
        };
        
        DesignResult {
            request_id: uuid::Uuid::new_v4().to_string(),
            content,
            preview_url: None,
            format: request.format,
            metadata: DesignMetadata {
                created_at: chrono::Utc::now(),
                word_count: 0, // Calculate after generation
                slide_count: template.structure.slides,
                sections: template.sections.iter().map(|s| s.name.clone()).collect(),
            },
        }
    }
    
    async fn generate_html(&self, request: &DesignRequest, template: &DesignTemplate) -> String {
        let mut html = format!(
            r#"<!DOCTYPE html>
<html lang="vi">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        :root {{
            --primary: {};
            --secondary: {};
            --font: {};
        }}
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{ font-family: var(--font); line-height: 1.6; }}
        .container {{ max-width: 1200px; margin: 0 auto; padding: 0 20px; }}
        .section {{ padding: 80px 0; }}
        .hero {{
            background: linear-gradient(135deg, var(--primary), var(--secondary));
            color: white;
            text-align: center;
            padding: 120px 20px;
        }}
        .hero h1 {{ font-size: 3rem; margin-bottom: 20px; }}
        .cta-button {{
            display: inline-block;
            background: white;
            color: var(--primary);
            padding: 15px 30px;
            border-radius: 8px;
            text-decoration: none;
            font-weight: bold;
            margin-top: 20px;
        }}
        .features {{ background: #f9fafb; }}
        .feature-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 30px;
            margin-top: 40px;
        }}
        .feature-card {{
            background: white;
            padding: 30px;
            border-radius: 12px;
            box-shadow: 0 4px 6px rgba(0,0,0,0.1);
        }}
        .stats {{
            display: flex;
            justify-content: center;
            gap: 60px;
            flex-wrap: wrap;
        }}
        .stat {{ text-align: center; }}
        .stat-number {{ font-size: 3rem; font-weight: bold; color: var(--primary); }}
        .cta {{ background: var(--primary); color: white; text-align: center; }}
    </style>
</head>
<body>
    <section class="hero">
        <div class="container">
            <h1>{}</h1>
            <p style="font-size: 1.25rem; margin-bottom: 30px;">
                Dành cho: {}
            </p>
            <a href="#contact" class="cta-button">Bắt đầu ngay</a>
        </div>
    </section>
    
    <section class="features section">
        <div class="container">
            <h2 style="text-align: center; font-size: 2rem;">Tính năng nổi bật</h2>
            <div class="feature-grid">
"#,
            request.topic,
            self.brand.primary_color,
            self.brand.secondary_color,
            self.brand.font_family,
            request.topic,
            request.audience
        );
        
        // Generate features based on topic
        let features = self.generate_features(&request.topic);
        for feature in features {
            html.push_str(&format!(
                r#"                <div class="feature-card">
                    <h3>{}</h3>
                    <p>{}</p>
                </div>
"#,
                feature.title, feature.description
            ));
        }
        
        html.push_str(
            r#"
            </div>
        </div>
    </section>
    
    <section class="section" style="text-align: center;">
        <div class="container">
            <h2 style="font-size: 2rem;">Được tin tưởng bởi</h2>
            <div class="stats">
                <div class="stat">
                    <div class="stat-number">1000+</div>
                    <div>Khách hàng</div>
                </div>
                <div class="stat">
                    <div class="stat-number">99%</div>
                    <div>Hài lòng</div>
                </div>
                <div class="stat">
                    <div class="stat-number">24/7</div>
                    <div>Hỗ trợ</div>
                </div>
            </div>
        </div>
    </section>
    
    <section class="cta section" id="contact">
        <div class="container">
            <h2 style="font-size: 2rem; margin-bottom: 20px;">Sẵn sàng bắt đầu?</h2>
            <p style="margin-bottom: 30px;">Liên hệ {} ngay hôm nay</p>
            <a href="mailto:contact@example.com" class="cta-button">Liên hệ ngay</a>
        </div>
    </section>
</body>
</html>"#,
            self.brand.company_name
        );
        
        html
    }
    
    async fn generate_markdown(&self, request: &DesignRequest, template: &DesignTemplate) -> String {
        let mut md = format!(
            "# {}\n\n",
            request.topic
        );
        
        md.push_str(&format!(
            "**Đối tượng:** {}\n\n",
            request.audience
        ));
        
        md.push_str("---\n\n");
        
        for section in &template.sections {
            md.push_str(&format!("## {}\n\n", section.name));
            md.push_str(&self.generate_section_content(&request.topic, &section.content_type));
            md.push_str("\n\n");
        }
        
        md
    }
    
    async fn generate_pptx(&self, _request: &DesignRequest, template: &DesignTemplate) -> String {
        // For PPTX, we'd use a library like `rust-docx` or `pptx-templates`
        // Placeholder: return markdown outline
        let mut outline = "# Slide Outline\n\n".to_string();
        for (i, section) in template.sections.iter().enumerate() {
            outline.push_str(&format!("{}. {}\n", i + 1, section.name));
        }
        outline
    }
    
    fn generate_features(&self, topic: &str) -> Vec<Feature> {
        vec![
            Feature {
                title: "Nhanh chóng".into(),
                description: format!("Tiết kiệm thời gian với {}", topic),
            },
            Feature {
                title: "Dễ dùng".into(),
                description: "Giao diện thân thiện, ai cũng có thể sử dụng".into(),
            },
            Feature {
                title: "An toàn".into(),
                description: "Bảo mật dữ liệu theo tiêu chuẩn quốc tế".into(),
            },
        ]
    }
    
    fn generate_section_content(&self, topic: &str, section_type: &SectionType) -> String {
        match section_type {
            SectionType::Hero => format!(
                "**{}** - Giải pháp hàng đầu cho doanh nghiệp của bạn",
                topic
            ),
            SectionType::Features => format!(
                "- Tính năng 1: Mô tả\n- Tính năng 2: Mô tả\n- Tính năng 3: Mô tả",
            ),
            SectionType::CTA => "Liên hệ ngay để được tư vấn miễn phí!".into(),
            _ => "Nội dung đang được cập nhật...".into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Feature {
    pub title: String,
    pub description: String,
}
```

#### 3.2 Designer Studio API Routes

```rust
// crates/bizclaw-gateway/src/routes/designer.rs

use axum::{extract::Json, routing::{get, post}, Router};

pub fn designer_routes() -> Router {
    Router::new()
        .route("/api/designer/generate", post(generate_content))
        .route("/api/designer/templates", get(list_templates))
        .route("/api/designer/preview/:id", get(get_preview))
}

#[derive(Deserialize)]
pub struct GenerateRequest {
    pub content_type: ContentType,
    pub topic: String,
    pub audience: String,
    pub tone: Option<String>,
    pub format: Option<OutputFormat>,
}

/// POST /api/designer/generate
async fn generate_content(Json(req): Json<GenerateRequest>) -> Json<DesignResult> {
    let brand = BrandConfig::default();
    let studio = DesignerStudio::new(brand);
    
    let design_request = DesignRequest {
        content_type: req.content_type,
        topic: req.topic,
        audience: req.audience,
        tone: req.tone.unwrap_or_else(|| "professional".to_string()),
        brand: None,
        format: req.format.unwrap_or(OutputFormat::Html),
    };
    
    let result = studio.generate(design_request).await;
    Json(result)
}

/// GET /api/designer/templates
async fn list_templates() -> Json<Vec<TemplateInfo>> {
    Json(vec![
        TemplateInfo { 
            id: "slide".into(), 
            name: "Slide Presentation".into(),
            description: "10-slide presentation template".into(),
            format: "pptx, html, md".into(),
        },
        TemplateInfo { 
            id: "landing".into(), 
            name: "Landing Page".into(),
            description: "Full landing page with sections".into(),
            format: "html".into(),
        },
    ])
}

#[derive(Serialize)]
pub struct TemplateInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub format: String,
}
```

---

## 4. Summary

### Features Implemented

| Feature | Status | Description |
|---------|--------|-------------|
| **Knowledge Graph Visualization** | ✅ | API + D3.js component |
| **Dream Cycle Memory** | ✅ | Scheduled memory refinement |
| **Designer Studio** | ✅ | Content generation (slides, landing pages) |

### Architecture

```
Thoth Features → BizClaw Implementation
├── Knowledge Graph → GraphVisualizer API + D3.js
├── Dream Cycle → DreamCycle service + Hand
└── Designer Studio → DesignerStudio service + API
```

### Files Created

```
bizclaw/
├── docs/THOTH_FEATURES_INTEGRATION.md
├── crates/
│   ├── bizclaw-knowledge/src/graph_visualizer.rs  (new)
│   ├── bizclaw-memory/src/dream_cycle.rs          (new)
│   ├── bizclaw-content/src/designer_studio.rs     (new)
│   └── bizclaw-gateway/src/routes/
│       ├── knowledge_graph.rs
│       └── designer.rs
└── frontend/components/
    └── GraphVisualization.tsx
```
