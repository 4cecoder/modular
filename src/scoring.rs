//! Scoring System
//!
//! A flexible scoring system that supports multiple score types, win conditions,
//! achievements, and scoring mechanics. Extracted and enhanced from the Pong game.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Score types for different game mechanics
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScoreType {
    Points,
    Kills,
    Deaths,
    Assists,
    Time,
    Distance,
    Accuracy,
    Combo,
    Custom(String),
}

/// Score entry for tracking individual scores
#[derive(Debug, Clone)]
pub struct ScoreEntry {
    pub player_id: String,
    pub score_type: ScoreType,
    pub value: i64,
    pub timestamp: Instant,
    pub metadata: HashMap<String, String>,
}

impl ScoreEntry {
    pub fn new(player_id: &str, score_type: ScoreType, value: i64) -> Self {
        Self {
            player_id: player_id.to_string(),
            score_type,
            value,
            timestamp: Instant::now(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    pub fn age(&self) -> Duration {
        self.timestamp.elapsed()
    }
}

/// Win condition types
#[derive(Debug, Clone)]
pub enum WinCondition {
    /// First to reach a score threshold
    ScoreThreshold { target_score: i64, score_type: ScoreType },
    /// Survive for a certain time
    TimeSurvival { duration: Duration },
    /// Eliminate all enemies
    Elimination,
    /// Reach a specific location
    Location { target_x: f32, target_y: f32, radius: f32 },
    /// Custom win condition
    Custom { condition_id: String },
}

/// Game result
#[derive(Debug, Clone)]
pub enum GameResult {
    Win { winner: String, reason: String },
    Loss { loser: String, reason: String },
    Draw { reason: String },
    Ongoing,
}

/// Achievement definition
#[derive(Debug, Clone)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub condition: AchievementCondition,
    pub reward_points: i64,
    pub unlocked: bool,
    pub unlocked_at: Option<Instant>,
}

#[derive(Debug, Clone)]
pub enum AchievementCondition {
    /// Reach a score threshold
    ScoreThreshold { score_type: ScoreType, threshold: i64 },
    /// Perform an action a certain number of times
    ActionCount { action: String, count: u32 },
    /// Complete a game with specific conditions
    GameCompletion { win_condition: WinCondition },
    /// Time-based achievement
    TimeBased { duration: Duration },
    /// Custom condition
    Custom { condition_id: String },
}

/// Main scoring system
pub struct ScoringSystem {
    scores: HashMap<String, HashMap<ScoreType, i64>>,
    score_history: Vec<ScoreEntry>,
    win_conditions: Vec<WinCondition>,
    achievements: HashMap<String, Achievement>,
    game_start_time: Instant,
    game_duration: Duration,
    max_history_size: usize,
}

impl ScoringSystem {
    /// Create a new scoring system
    pub fn new() -> Self {
        Self {
            scores: HashMap::new(),
            score_history: Vec::new(),
            win_conditions: Vec::new(),
            achievements: HashMap::new(),
            game_start_time: Instant::now(),
            game_duration: Duration::from_secs(0),
            max_history_size: 1000,
        }
    }

    /// Create a scoring system with Pong defaults
    pub fn with_pong_defaults(max_score: i32) -> Self {
        let mut system = Self::new();
        system.add_win_condition(WinCondition::ScoreThreshold {
            target_score: max_score as i64,
            score_type: ScoreType::Points,
        });
        system
    }

    /// Add points to a player's score
    pub fn add_score(&mut self, player_id: &str, score_type: ScoreType, points: i64) -> i64 {
        let player_scores = self.scores.entry(player_id.to_string()).or_insert_with(HashMap::new);
        let current_score = player_scores.entry(score_type.clone()).or_insert(0);
        *current_score += points;

        // Record in history
        let entry = ScoreEntry::new(player_id, score_type, points);
        self.score_history.push(entry);

        // Trim history if too large
        if self.score_history.len() > self.max_history_size {
            self.score_history.remove(0);
        }

        let final_score = *current_score;

        // Check achievements (separate scope to avoid borrowing issues)
        self.check_achievements(player_id);

        final_score
    }

    /// Get a player's score for a specific type
    pub fn get_score(&self, player_id: &str, score_type: &ScoreType) -> i64 {
        self.scores
            .get(player_id)
            .and_then(|player_scores| player_scores.get(score_type))
            .copied()
            .unwrap_or(0)
    }

    /// Get all scores for a player
    pub fn get_player_scores(&self, player_id: &str) -> HashMap<ScoreType, i64> {
        self.scores
            .get(player_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Get the leaderboard for a specific score type
    pub fn get_leaderboard(&self, score_type: &ScoreType) -> Vec<(String, i64)> {
        let mut leaderboard: Vec<(String, i64)> = self.scores
            .iter()
            .filter_map(|(player_id, scores)| {
                scores.get(score_type).map(|&score| (player_id.clone(), score))
            })
            .collect();

        leaderboard.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by score descending
        leaderboard
    }

    /// Add a win condition
    pub fn add_win_condition(&mut self, condition: WinCondition) {
        self.win_conditions.push(condition);
    }

    /// Check if any win condition is met
    pub fn check_win_conditions(&self) -> GameResult {
        for condition in &self.win_conditions {
            match condition {
                WinCondition::ScoreThreshold { target_score, score_type } => {
                    for (player_id, scores) in &self.scores {
                        if let Some(&score) = scores.get(score_type) {
                            if score >= *target_score {
                                return GameResult::Win {
                                    winner: player_id.clone(),
                                    reason: format!("Reached {} {}", target_score, format_score_type(score_type)),
                                };
                            }
                        }
                    }
                },
                WinCondition::TimeSurvival { duration } => {
                    if self.game_duration >= *duration {
                        return GameResult::Win {
                            winner: "Player".to_string(), // Could be made more flexible
                            reason: format!("Survived for {:?}", duration),
                        };
                    }
                },
                WinCondition::Elimination => {
                    // Check if all enemies are eliminated
                    // This would need to be integrated with the game state
                    // For now, return Ongoing
                },
                WinCondition::Location { target_x, target_y, radius } => {
                    // Check if any player reached the location
                    // This would need position data from the game
                    // For now, return Ongoing
                },
                WinCondition::Custom { condition_id } => {
                    // Custom conditions would need to be checked by the game
                    // For now, return Ongoing
                },
            }
        }

        GameResult::Ongoing
    }

    /// Add an achievement
    pub fn add_achievement(&mut self, achievement: Achievement) {
        self.achievements.insert(achievement.id.clone(), achievement);
    }

    /// Check achievements for a player
    pub fn check_achievements(&mut self, player_id: &str) {
        let mut achievements_to_unlock = Vec::new();

        // First pass: collect achievements that should be unlocked
        for (id, achievement) in &self.achievements {
            if achievement.unlocked {
                continue;
            }

            let should_unlock = match &achievement.condition {
                AchievementCondition::ScoreThreshold { score_type, threshold } => {
                    self.get_score(player_id, score_type) >= *threshold
                },
                AchievementCondition::ActionCount { action, count } => {
                    // Count actions from history
                    let action_count = self.score_history
                        .iter()
                        .filter(|entry| {
                            entry.player_id == player_id &&
                            entry.metadata.get("action").map(|a| a == action).unwrap_or(false)
                        })
                        .count();
                    action_count >= *count as usize
                },
                AchievementCondition::GameCompletion { .. } => {
                    // Would need game completion data
                    false
                },
                AchievementCondition::TimeBased { duration } => {
                    self.game_duration >= *duration
                },
                AchievementCondition::Custom { .. } => {
                    // Custom conditions would need to be checked by the game
                    false
                },
            };

            if should_unlock {
                achievements_to_unlock.push(id.clone());
            }
        }

        // Second pass: unlock achievements
        for achievement_id in achievements_to_unlock {
            if let Some(achievement) = self.achievements.get_mut(&achievement_id) {
                achievement.unlocked = true;
                achievement.unlocked_at = Some(Instant::now());
                // Could trigger some callback here
            }
        }
    }

    /// Get unlocked achievements for a player
    pub fn get_unlocked_achievements(&self, player_id: &str) -> Vec<&Achievement> {
        self.achievements
            .values()
            .filter(|achievement| achievement.unlocked)
            .collect()
    }

    /// Update game time
    pub fn update_time(&mut self, delta_time: f32) {
        self.game_duration = self.game_start_time.elapsed();
    }

    /// Reset the scoring system
    pub fn reset(&mut self) {
        self.scores.clear();
        self.score_history.clear();
        self.game_start_time = Instant::now();
        self.game_duration = Duration::from_secs(0);

        // Reset achievements but keep definitions
        for achievement in self.achievements.values_mut() {
            achievement.unlocked = false;
            achievement.unlocked_at = None;
        }
    }

    /// Get game statistics
    pub fn get_stats(&self) -> GameStats {
        let total_scores: i64 = self.score_history.iter().map(|entry| entry.value).sum();
        let total_actions = self.score_history.len();

        GameStats {
            total_scores,
            total_actions,
            game_duration: self.game_duration,
            active_players: self.scores.len(),
            unlocked_achievements: self.achievements.values().filter(|a| a.unlocked).count(),
        }
    }

    /// Export scores to a serializable format
    pub fn export_scores(&self) -> Vec<SerializableScoreEntry> {
        self.score_history
            .iter()
            .map(|entry| SerializableScoreEntry {
                player_id: entry.player_id.clone(),
                score_type: format!("{:?}", entry.score_type),
                value: entry.value,
                timestamp: entry.timestamp.elapsed().as_secs(),
                metadata: entry.metadata.clone(),
            })
            .collect()
    }
}

/// Game statistics
#[derive(Debug, Clone)]
pub struct GameStats {
    pub total_scores: i64,
    pub total_actions: usize,
    pub game_duration: Duration,
    pub active_players: usize,
    pub unlocked_achievements: usize,
}

/// Serializable score entry for saving/loading
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SerializableScoreEntry {
    pub player_id: String,
    pub score_type: String,
    pub value: i64,
    pub timestamp: u64,
    pub metadata: HashMap<String, String>,
}

impl Default for ScoringSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper functions
fn format_score_type(score_type: &ScoreType) -> String {
    match score_type {
        ScoreType::Points => "points".to_string(),
        ScoreType::Kills => "kills".to_string(),
        ScoreType::Deaths => "deaths".to_string(),
        ScoreType::Assists => "assists".to_string(),
        ScoreType::Time => "time".to_string(),
        ScoreType::Distance => "distance".to_string(),
        ScoreType::Accuracy => "accuracy".to_string(),
        ScoreType::Combo => "combo".to_string(),
        ScoreType::Custom(name) => name.clone(),
    }
}

/// Preset scoring configurations
pub mod presets {
    use super::*;

    /// Create a Pong scoring system
    pub fn pong_scoring(max_score: i32) -> ScoringSystem {
        let mut system = ScoringSystem::with_pong_defaults(max_score);

        // Add Pong-specific achievements
        system.add_achievement(Achievement {
            id: "first_point".to_string(),
            name: "First Blood".to_string(),
            description: "Score your first point".to_string(),
            condition: AchievementCondition::ScoreThreshold {
                score_type: ScoreType::Points,
                threshold: 1,
            },
            reward_points: 10,
            unlocked: false,
            unlocked_at: None,
        });

        system.add_achievement(Achievement {
            id: "speed_demon".to_string(),
            name: "Speed Demon".to_string(),
            description: "Win a game in under 2 minutes".to_string(),
            condition: AchievementCondition::TimeBased {
                duration: std::time::Duration::from_secs(120),
            },
            reward_points: 50,
            unlocked: false,
            unlocked_at: None,
        });

        system
    }

    /// Create a shooter scoring system
    pub fn shooter_scoring() -> ScoringSystem {
        let mut system = ScoringSystem::new();

        system.add_win_condition(WinCondition::Elimination);

        system.add_achievement(Achievement {
            id: "headshot_master".to_string(),
            name: "Headshot Master".to_string(),
            description: "Get 10 headshots".to_string(),
            condition: AchievementCondition::ActionCount {
                action: "headshot".to_string(),
                count: 10,
            },
            reward_points: 100,
            unlocked: false,
            unlocked_at: None,
        });

        system
    }

    /// Create a racing scoring system
    pub fn racing_scoring() -> ScoringSystem {
        let mut system = ScoringSystem::new();

        system.add_win_condition(WinCondition::Location {
            target_x: 1000.0,
            target_y: 500.0,
            radius: 50.0,
        });

        system.add_achievement(Achievement {
            id: "speed_racer".to_string(),
            name: "Speed Racer".to_string(),
            description: "Complete a lap in under 30 seconds".to_string(),
            condition: AchievementCondition::TimeBased {
                duration: std::time::Duration::from_secs(30),
            },
            reward_points: 75,
            unlocked: false,
            unlocked_at: None,
        });

        system
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scoring_system() {
        let mut system = ScoringSystem::new();

        system.add_score("player1", ScoreType::Points, 10);
        assert_eq!(system.get_score("player1", &ScoreType::Points), 10);

        system.add_score("player1", ScoreType::Points, 5);
        assert_eq!(system.get_score("player1", &ScoreType::Points), 15);
    }

    #[test]
    fn test_win_conditions() {
        let mut system = ScoringSystem::with_pong_defaults(5);

        system.add_score("player1", ScoreType::Points, 5);

        match system.check_win_conditions() {
            GameResult::Win { winner, .. } => assert_eq!(winner, "player1"),
            _ => panic!("Expected win condition"),
        }
    }

    #[test]
    fn test_leaderboard() {
        let mut system = ScoringSystem::new();

        system.add_score("player1", ScoreType::Points, 10);
        system.add_score("player2", ScoreType::Points, 20);
        system.add_score("player3", ScoreType::Points, 15);

        let leaderboard = system.get_leaderboard(&ScoreType::Points);
        assert_eq!(leaderboard[0], ("player2".to_string(), 20));
        assert_eq!(leaderboard[1], ("player3".to_string(), 15));
        assert_eq!(leaderboard[2], ("player1".to_string(), 10));
    }

    #[test]
    fn test_achievements() {
        let mut system = ScoringSystem::new();

        let achievement = Achievement {
            id: "test_achievement".to_string(),
            name: "Test Achievement".to_string(),
            description: "Test achievement".to_string(),
            condition: AchievementCondition::ScoreThreshold {
                score_type: ScoreType::Points,
                threshold: 5,
            },
            reward_points: 10,
            unlocked: false,
            unlocked_at: None,
        };

        system.add_achievement(achievement);
        system.add_score("player1", ScoreType::Points, 5);

        let unlocked = system.get_unlocked_achievements("player1");
        assert_eq!(unlocked.len(), 1);
        assert_eq!(unlocked[0].id, "test_achievement");
    }

    #[test]
    fn test_pong_preset() {
        let system = presets::pong_scoring(5);
        assert_eq!(system.win_conditions.len(), 1);
        assert!(!system.achievements.is_empty());
    }
}