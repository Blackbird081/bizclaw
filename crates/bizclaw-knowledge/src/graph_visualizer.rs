//! # Knowledge Graph Visualizer
//!
//! Provides API endpoints and data structures for visualizing the knowledge graph
//! in frontend dashboards using D3.js or similar visualization libraries.
//!
//! ## Features
//! - Entity-to-visualization format conversion
//! - Node coloring by entity type
//! - Edge styling by relationship weight
//! - Hierarchical layout support
//! - Statistics and aggregations

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::graph::{Entity, EntityType, KnowledgeGraph, RelationType, Relationship};

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
    pub entity_type: String,
}

impl From<&Entity> for GraphNode {
    fn from(entity: &Entity) -> Self {
        let type_str = entity.entity_type.as_str();
        Self {
            id: entity.id.0.clone(),
            label: entity.name.clone(),
            group: type_str.to_string(),
            title: entity.description.clone().unwrap_or_default(),
            level: 0,
            size: 1.0 + (entity.properties.len() as f32 * 0.1),
            color: Self::type_to_color(&entity.entity_type),
            image: None,
            shape: Self::type_to_shape(&entity.entity_type),
            entity_type: type_str.to_string(),
        }
    }
}

impl GraphNode {
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

    fn type_to_shape(entity_type: &EntityType) -> String {
        match entity_type {
            EntityType::Person => "circularImage".to_string(),
            EntityType::Organization => "box".to_string(),
            EntityType::Product => "diamond".to_string(),
            EntityType::Concept => "ellipse".to_string(),
            EntityType::Location => "star".to_string(),
            EntityType::Event => "triangle".to_string(),
            EntityType::Document => "text".to_string(),
            EntityType::Task => "square".to_string(),
            EntityType::Custom(_) => "dot".to_string(),
        }
    }
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
    pub relation_type: String,
}

impl From<&Relationship> for GraphEdge {
    fn from(rel: &Relationship) -> Self {
        Self {
            id: rel.id.clone(),
            from: rel.source.0.clone(),
            to: rel.target.0.clone(),
            label: rel.relation_type.as_str().to_string(),
            arrows: "to".to_string(),
            dashes: rel.weight < 0.5,
            width: 1.0 + (rel.weight * 3.0),
            color: Self::weight_to_color(rel.weight),
            relation_type: rel.relation_type.as_str().to_string(),
        }
    }
}

impl GraphEdge {
    fn weight_to_color(weight: f32) -> String {
        if weight > 0.7 {
            "#27AE60".to_string() // Strong - green
        } else if weight > 0.4 {
            "#F39C12".to_string() // Medium - orange
        } else {
            "#E74C3C".to_string() // Weak - red
        }
    }
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
    pub top_relations: Vec<RelationStat>,
    pub orphaned_entities: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationStat {
    pub relation_type: String,
    pub count: usize,
}

/// Query options for graph visualization
#[derive(Debug, Clone, Deserialize)]
pub struct GraphQuery {
    pub entity_types: Option<Vec<String>>,
    pub min_weight: Option<f32>,
    pub limit: Option<usize>,
    pub depth: Option<usize>,
}

impl Default for GraphQuery {
    fn default() -> Self {
        Self {
            entity_types: None,
            min_weight: None,
            limit: Some(100),
            depth: Some(1),
        }
    }
}

/// Entity detail response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDetail {
    pub entity: GraphNode,
    pub incoming: Vec<RelationshipInfo>,
    pub outgoing: Vec<RelationshipInfo>,
    pub neighbors: Vec<GraphNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipInfo {
    pub id: String,
    pub source: String,
    pub target: String,
    pub relation_type: String,
    pub weight: f32,
    pub properties: HashMap<String, String>,
}

impl From<&Relationship> for RelationshipInfo {
    fn from(rel: &Relationship) -> Self {
        Self {
            id: rel.id.clone(),
            source: rel.source.0.clone(),
            target: rel.target.0.clone(),
            relation_type: rel.relation_type.as_str().to_string(),
            weight: rel.weight,
            properties: rel.properties.clone(),
        }
    }
}

/// Path between two entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityPath {
    pub path: Vec<String>,
    pub distance: usize,
    pub entities: Vec<GraphNode>,
    pub relationships: Vec<RelationshipInfo>,
}

impl KnowledgeGraph {
    /// Convert to visualization format
    pub async fn to_visualization(&self, query: &GraphQuery) -> GraphVisualization {
        let entities = self.entities.read().await;
        let relationships = self.relationships.read().await;

        // Filter entities by type
        let filtered_entities: Vec<&Entity> = if let Some(ref types) = query.entity_types {
            entities
                .values()
                .filter(|e| types.contains(&e.entity_type.as_str().to_string()))
                .collect()
        } else {
            entities.values().collect()
        };

        // Build nodes
        let mut nodes: Vec<GraphNode> = Vec::new();
        let mut entity_type_counts: HashMap<String, usize> = HashMap::new();
        let limit = query.limit.unwrap_or(100);

        for entity in filtered_entities.iter().take(limit) {
            let type_str = entity.entity_type.as_str().to_string();
            *entity_type_counts.entry(type_str.clone()).or_insert(0) += 1;
            nodes.push(GraphNode::from(*entity));
        }

        // Build entity ID set for filtering relationships
        let entity_ids: HashSet<String> = nodes.iter().map(|n| n.id.clone()).collect();

        // Build edges
        let mut edges: Vec<GraphEdge> = Vec::new();
        let mut relation_counts: HashMap<String, usize> = HashMap::new();

        for rel in relationships.values() {
            // Filter by min_weight
            if let Some(min) = query.min_weight {
                if rel.weight < min {
                    continue;
                }
            }

            // Only include edges where both endpoints are in filtered entities
            if entity_ids.contains(&rel.source.0) && entity_ids.contains(&rel.target.0) {
                let rel_type = rel.relation_type.as_str().to_string();
                *relation_counts.entry(rel_type.clone()).or_insert(0) += 1;
                edges.push(GraphEdge::from(rel));
            }
        }

        // Count orphaned entities
        let mut connected: HashSet<String> = HashSet::new();
        for rel in relationships.values() {
            connected.insert(rel.source.0.clone());
            connected.insert(rel.target.0.clone());
        }
        let orphaned_count = entities
            .values()
            .filter(|e| !connected.contains(&e.id.0))
            .count();

        // Top relations
        let mut top_relations: Vec<RelationStat> = relation_counts
            .into_iter()
            .map(|(relation_type, count)| RelationStat {
                relation_type,
                count,
            })
            .collect();
        top_relations.sort_by(|a, b| b.count.cmp(&a.count));
        top_relations.truncate(10);

        GraphVisualization {
            nodes,
            edges,
            stats: GraphStats {
                total_entities: entities.len(),
                total_relationships: relationships.len(),
                entity_types: entity_type_counts,
                top_relations,
                orphaned_entities: orphaned_count,
            },
        }
    }

    /// Get entity detail with relationships
    pub async fn get_entity_detail(&self, entity_id: &str) -> Option<EntityDetail> {
        let entities = self.entities.read().await;
        let relationships = self.relationships.read().await;

        let entity = entities.get(&crate::graph::EntityId(entity_id.to_string()))?;

        let mut incoming: Vec<RelationshipInfo> = Vec::new();
        let mut outgoing: Vec<RelationshipInfo> = Vec::new();

        for rel in relationships.values() {
            if rel.target.0 == entity_id {
                incoming.push(RelationshipInfo::from(rel));
            }
            if rel.source.0 == entity_id {
                outgoing.push(RelationshipInfo::from(rel));
            }
        }

        // Get neighbors (entities connected to this entity)
        let neighbor_ids: HashSet<String> = relationships
            .values()
            .filter(|r| r.source.0 == entity_id || r.target.0 == entity_id)
            .flat_map(|r| vec![r.source.0.clone(), r.target.0.clone()])
            .filter(|id| id != entity_id)
            .collect();

        let neighbors: Vec<GraphNode> = neighbor_ids
            .iter()
            .filter_map(|id| entities.get(&crate::graph::EntityId(id.clone())))
            .map(GraphNode::from)
            .collect();

        Some(EntityDetail {
            entity: GraphNode::from(entity),
            incoming,
            outgoing,
            neighbors,
        })
    }

    /// Search entities by query
    pub async fn search_entities(&self, query: &str, limit: usize) -> Vec<GraphNode> {
        let entities = self.entities.read().await;
        let query_lower = query.to_lowercase();

        let mut results: Vec<(f64, &Entity)> = entities
            .values()
            .filter_map(|entity| {
                let mut score = 0.0;

                // Name match (highest priority)
                if entity.name.to_lowercase().contains(&query_lower) {
                    score += 10.0;
                }

                // Description match
                if let Some(ref desc) = entity.description {
                    if desc.to_lowercase().contains(&query_lower) {
                        score += 5.0;
                    }
                }

                // Type match
                if entity.entity_type.as_str().contains(&query_lower) {
                    score += 3.0;
                }

                // Property match
                for (_, value) in &entity.properties {
                    if value.to_lowercase().contains(&query_lower) {
                        score += 1.0;
                    }
                }

                if score > 0.0 {
                    Some((score, *entity))
                } else {
                    None
                }
            })
            .collect();

        results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        results
            .into_iter()
            .take(limit)
            .map(|(_, e)| GraphNode::from(e))
            .collect()
    }

    /// Find path between two entities using BFS
    pub async fn find_path(&self, from_id: &str, to_id: &str) -> Option<EntityPath> {
        use std::collections::VecDeque;

        let relationships = self.relationships.read().await;

        // Build adjacency list
        let mut adj: HashMap<String, Vec<String>> = HashMap::new();
        for rel in relationships.values() {
            adj.entry(rel.source.0.clone())
                .or_default()
                .push(rel.target.0.clone());
            adj.entry(rel.target.0.clone())
                .or_default()
                .push(rel.source.0.clone());
        }

        // BFS
        let mut queue: VecDeque<(String, Vec<String>)> = VecDeque::new();
        let mut visited: HashSet<String> = HashSet::new();

        queue.push_back((from_id.to_string(), vec![from_id.to_string()]));
        visited.insert(from_id.to_string());

        while let Some((current, path)) = queue.pop_front() {
            if current == to_id {
                // Found path - return entity details
                let entities = self.entities.read().await;
                let entity_nodes: Vec<GraphNode> = path
                    .iter()
                    .filter_map(|id| {
                        entities
                            .get(&crate::graph::EntityId(id.clone()))
                            .map(GraphNode::from)
                    })
                    .collect();

                let rel_infos: Vec<RelationshipInfo> = relationships
                    .values()
                    .filter(|r| {
                        let from_in_path = path.contains(&r.source.0);
                        let to_in_path = path.contains(&r.target.0);
                        // Check if this relationship connects consecutive nodes in path
                        from_in_path
                            && to_in_path
                            && path
                                .windows(2)
                                .any(|w| w[0] == r.source.0 && w[1] == r.target.0 || w[0] == r.target.0 && w[1] == r.source.0)
                    })
                    .map(RelationshipInfo::from)
                    .collect();

                return Some(EntityPath {
                    path: path.clone(),
                    distance: path.len() - 1,
                    entities: entity_nodes,
                    relationships: rel_infos,
                });
            }

            if let Some(neighbors) = adj.get(&current) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        let mut new_path = path.clone();
                        new_path.push(neighbor.clone());
                        queue.push_back((neighbor.clone(), new_path));
                    }
                }
            }
        }

        None
    }

    /// Get entity types with counts
    pub async fn get_entity_types(&self) -> HashMap<String, usize> {
        let entities = self.entities.read().await;
        let mut counts: HashMap<String, usize> = HashMap::new();

        for entity in entities.values() {
            let type_str = entity.entity_type.as_str().to_string();
            *counts.entry(type_str).or_insert(0) += 1;
        }

        counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_to_visualization() {
        let graph = KnowledgeGraph::new();

        // Add some entities
        let person_id = graph
            .add_entity(Entity {
                id: crate::graph::EntityId("person1".to_string()),
                name: "Nguyen Van A".to_string(),
                entity_type: EntityType::Person,
                properties: HashMap::new(),
                description: Some("A person".to_string()),
                embedding: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
            .await;

        let org_id = graph
            .add_entity(Entity {
                id: crate::graph::EntityId("org1".to_string()),
                name: "Cong ty ABC".to_string(),
                entity_type: EntityType::Organization,
                properties: HashMap::new(),
                description: Some("A company".to_string()),
                embedding: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
            .await;

        // Add relationship
        graph
            .add_relationship(Relationship {
                id: "rel1".to_string(),
                source: person_id.clone(),
                target: org_id.clone(),
                relation_type: RelationType::WorksFor,
                properties: HashMap::new(),
                weight: 0.8,
                created_at: chrono::Utc::now(),
            })
            .await;

        let query = GraphQuery::default();
        let viz = graph.to_visualization(&query).await;

        assert_eq!(viz.nodes.len(), 2);
        assert_eq!(viz.edges.len(), 1);
        assert_eq!(viz.stats.total_entities, 2);
        assert_eq!(viz.stats.total_relationships, 1);
    }

    #[tokio::test]
    async fn test_search_entities() {
        let graph = KnowledgeGraph::new();

        graph
            .add_entity(Entity {
                id: crate::graph::EntityId("1".to_string()),
                name: "Nguyen Van A".to_string(),
                entity_type: EntityType::Person,
                properties: HashMap::new(),
                description: Some("Developer".to_string()),
                embedding: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
            .await;

        let results = graph.search_entities("nguyen", 10).await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].label, "Nguyen Van A");
    }

    #[tokio::test]
    async fn test_find_path() {
        let graph = KnowledgeGraph::new();

        // Create a chain: A -> B -> C -> D
        let ids = vec!["a", "b", "c", "d"];
        for (i, id) in ids.iter().enumerate() {
            graph
                .add_entity(Entity {
                    id: crate::graph::EntityId(id.to_string()),
                    name: format!("Entity {}", id),
                    entity_type: EntityType::Concept,
                    properties: HashMap::new(),
                    description: None,
                    embedding: None,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                })
                .await;
        }

        // Add relationships
        graph
            .add_relationship(Relationship {
                id: "r1".to_string(),
                source: crate::graph::EntityId("a".to_string()),
                target: crate::graph::EntityId("b".to_string()),
                relation_type: RelationType::RelatedTo,
                properties: HashMap::new(),
                weight: 1.0,
                created_at: chrono::Utc::now(),
            })
            .await;

        graph
            .add_relationship(Relationship {
                id: "r2".to_string(),
                source: crate::graph::EntityId("b".to_string()),
                target: crate::graph::EntityId("c".to_string()),
                relation_type: RelationType::RelatedTo,
                properties: HashMap::new(),
                weight: 1.0,
                created_at: chrono::Utc::now(),
            })
            .await;

        graph
            .add_relationship(Relationship {
                id: "r3".to_string(),
                source: crate::graph::EntityId("c".to_string()),
                target: crate::graph::EntityId("d".to_string()),
                relation_type: RelationType::RelatedTo,
                properties: HashMap::new(),
                weight: 1.0,
                created_at: chrono::Utc::now(),
            })
            .await;

        let path = graph.find_path("a", "d").await;
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path.distance, 3);
        assert_eq!(path.path, vec!["a", "b", "c", "d"]);
    }
}
