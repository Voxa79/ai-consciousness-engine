// Multimodal Interface - Système d'Interface Multimodale Consciousness-Level
// Interface révolutionnaire combinant voix, vision, biométrie, et spatial computing

pub mod voice_processing;
pub mod vision_processing;
pub mod biometric_integration;
pub mod spatial_computing;
pub mod fusion_engine;
pub mod interaction_orchestrator;

pub use voice_processing::*;
pub use vision_processing::*;
pub use biometric_integration::*;
pub use spatial_computing::*;
pub use fusion_engine::*;
pub use interaction_orchestrator::*;

use crate::error::ConsciousnessResult;
use crate::emotions::{EmotionalState, EmotionalContext};
use crate::modules::SelfAwareness;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::time::Instant;

/// Types de modalités d'interaction
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum ModalityType {
    Voice,          // Modalité vocale
    Vision,         // Modalité visuelle
    Biometric,      // Modalité biométrique
    Spatial,        // Modalité spatiale
    Haptic,         // Modalité haptique
    Environmental,  // Modalité environnementale
}

/// Données d'entrée multimodales
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultimodalInput {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub voice_data: Option<VoiceData>,
    pub vision_data: Option<VisionData>,
    pub biometric_data: Option<BiometricData>,
    pub spatial_data: Option<SpatialData>,
    pub environmental_data: Option<EnvironmentalData>,
    pub context: InteractionContext,
}

/// Contexte d'interaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionContext {
    pub user_id: String,
    pub session_id: String,
    pub interaction_history: Vec<InteractionEvent>,
    pub user_preferences: UserPreferences,
    pub environmental_conditions: EnvironmentalConditions,
    pub privacy_settings: PrivacySettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionEvent {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub event_type: String,
    pub modalities_involved: Vec<ModalityType>,
    pub success_score: f64,
    pub user_satisfaction: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub preferred_modalities: Vec<ModalityType>,
    pub interaction_style: String,
    pub privacy_level: f64,
    pub accessibility_needs: Vec<String>,
    pub cultural_preferences: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalConditions {
    pub lighting_level: f64,
    pub noise_level: f64,
    pub temperature: f64,
    pub humidity: f64,
    pub air_quality: f64,
    pub other_people_present: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacySettings {
    pub voice_recording_allowed: bool,
    pub video_recording_allowed: bool,
    pub biometric_monitoring_allowed: bool,
    pub data_retention_period: std::time::Duration,
    pub sharing_permissions: HashMap<String, bool>,
}

/// Réponse multimodale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultimodalResponse {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub voice_response: Option<VoiceResponse>,
    pub visual_response: Option<VisualResponse>,
    pub haptic_response: Option<HapticResponse>,
    pub spatial_response: Option<SpatialResponse>,
    pub environmental_response: Option<EnvironmentalResponse>,
    pub coordination_metadata: CoordinationMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationMetadata {
    pub synchronization_timing: HashMap<ModalityType, std::time::Duration>,
    pub coherence_score: f64,
    pub adaptation_applied: Vec<String>,
    pub fallback_strategies: Vec<String>,
    pub quality_metrics: QualityMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    pub overall_quality: f64,
    pub modality_quality: HashMap<ModalityType, f64>,
    pub user_experience_score: f64,
    pub technical_performance: f64,
    pub accessibility_compliance: f64,
}

/// Données vocales
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceData {
    pub audio_stream: Vec<u8>,
    pub sample_rate: u32,
    pub channels: u8,
    pub duration: std::time::Duration,
    pub language_hint: Option<String>,
    pub speaker_characteristics: SpeakerCharacteristics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakerCharacteristics {
    pub estimated_age_range: (u8, u8),
    pub estimated_gender: Option<String>,
    pub accent_indicators: Vec<String>,
    pub emotional_markers: Vec<String>,
    pub speech_patterns: Vec<String>,
}

/// Données visuelles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionData {
    pub image_data: Vec<u8>,
    pub video_stream: Option<Vec<u8>>,
    pub resolution: (u32, u32),
    pub color_space: String,
    pub frame_rate: Option<f32>,
    pub camera_metadata: CameraMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraMetadata {
    pub camera_type: String,
    pub focal_length: Option<f32>,
    pub aperture: Option<f32>,
    pub iso: Option<u32>,
    pub white_balance: Option<String>,
    pub exposure_time: Option<std::time::Duration>,
}

/// Données biométriques
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricData {
    pub heart_rate: Option<f64>,
    pub heart_rate_variability: Option<f64>,
    pub skin_conductance: Option<f64>,
    pub body_temperature: Option<f64>,
    pub breathing_rate: Option<f64>,
    pub eye_tracking: Option<EyeTrackingData>,
    pub facial_micro_expressions: Option<Vec<MicroExpression>>,
    pub posture_data: Option<PostureData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EyeTrackingData {
    pub gaze_point: (f64, f64),
    pub pupil_diameter: f64,
    pub blink_rate: f64,
    pub saccade_patterns: Vec<SaccadePattern>,
    pub fixation_duration: std::time::Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaccadePattern {
    pub start_point: (f64, f64),
    pub end_point: (f64, f64),
    pub velocity: f64,
    pub duration: std::time::Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicroExpression {
    pub expression_type: String,
    pub intensity: f64,
    pub duration: std::time::Duration,
    pub facial_regions: Vec<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostureData {
    pub body_position: String,
    pub posture_score: f64,
    pub tension_areas: Vec<String>,
    pub movement_patterns: Vec<MovementPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementPattern {
    pub movement_type: String,
    pub frequency: f64,
    pub amplitude: f64,
    pub rhythm: Option<f64>,
}

/// Données spatiales
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialData {
    pub position_3d: (f64, f64, f64),
    pub orientation: (f64, f64, f64),
    pub movement_vector: (f64, f64, f64),
    pub proximity_objects: Vec<ProximityObject>,
    pub spatial_boundaries: SpatialBoundaries,
    pub interaction_zones: Vec<InteractionZone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProximityObject {
    pub object_id: String,
    pub object_type: String,
    pub distance: f64,
    pub relative_position: (f64, f64, f64),
    pub interaction_affordances: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialBoundaries {
    pub room_dimensions: (f64, f64, f64),
    pub safe_zones: Vec<SafeZone>,
    pub restricted_areas: Vec<RestrictedArea>,
    pub navigation_paths: Vec<NavigationPath>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafeZone {
    pub zone_id: String,
    pub boundaries: Vec<(f64, f64, f64)>,
    pub safety_level: f64,
    pub allowed_activities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestrictedArea {
    pub area_id: String,
    pub boundaries: Vec<(f64, f64, f64)>,
    pub restriction_type: String,
    pub access_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationPath {
    pub path_id: String,
    pub waypoints: Vec<(f64, f64, f64)>,
    pub path_type: String,
    pub difficulty_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionZone {
    pub zone_id: String,
    pub center_point: (f64, f64, f64),
    pub radius: f64,
    pub interaction_types: Vec<String>,
    pub optimal_positioning: Vec<(f64, f64, f64)>,
}

/// Données environnementales
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalData {
    pub ambient_light: LightingData,
    pub acoustic_environment: AcousticData,
    pub air_quality: AirQualityData,
    pub electromagnetic_environment: ElectromagneticData,
    pub social_presence: SocialPresenceData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightingData {
    pub illuminance: f64,
    pub color_temperature: f64,
    pub light_sources: Vec<LightSource>,
    pub shadows: Vec<ShadowArea>,
    pub glare_assessment: GlareAssessment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightSource {
    pub source_type: String,
    pub position: (f64, f64, f64),
    pub intensity: f64,
    pub color_spectrum: Vec<f64>,
    pub directional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowArea {
    pub area_boundaries: Vec<(f64, f64, f64)>,
    pub shadow_intensity: f64,
    pub shadow_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlareAssessment {
    pub glare_level: f64,
    pub glare_sources: Vec<String>,
    pub discomfort_probability: f64,
    pub mitigation_suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcousticData {
    pub ambient_noise_level: f64,
    pub frequency_spectrum: Vec<f64>,
    pub noise_sources: Vec<NoiseSource>,
    pub reverberation_time: f64,
    pub speech_intelligibility: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoiseSource {
    pub source_type: String,
    pub position: Option<(f64, f64, f64)>,
    pub intensity: f64,
    pub frequency_profile: Vec<f64>,
    pub intermittency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirQualityData {
    pub co2_level: f64,
    pub humidity: f64,
    pub temperature: f64,
    pub particulate_matter: HashMap<String, f64>,
    pub volatile_compounds: HashMap<String, f64>,
    pub air_circulation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectromagneticData {
    pub wifi_signals: Vec<WifiSignal>,
    pub bluetooth_devices: Vec<BluetoothDevice>,
    pub cellular_signals: Vec<CellularSignal>,
    pub electromagnetic_interference: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiSignal {
    pub ssid: String,
    pub signal_strength: f64,
    pub frequency: f64,
    pub security_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BluetoothDevice {
    pub device_name: String,
    pub device_type: String,
    pub signal_strength: f64,
    pub connection_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellularSignal {
    pub carrier: String,
    pub signal_strength: f64,
    pub network_type: String,
    pub data_speed: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialPresenceData {
    pub people_count: u32,
    pub people_positions: Vec<PersonPosition>,
    pub social_dynamics: SocialDynamics,
    pub privacy_implications: PrivacyImplications,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonPosition {
    pub person_id: Option<String>,
    pub position: (f64, f64, f64),
    pub orientation: (f64, f64, f64),
    pub activity: String,
    pub attention_direction: Option<(f64, f64, f64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialDynamics {
    pub interaction_patterns: Vec<String>,
    pub group_formations: Vec<GroupFormation>,
    pub communication_flows: Vec<CommunicationFlow>,
    pub social_roles: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupFormation {
    pub group_id: String,
    pub members: Vec<String>,
    pub formation_type: String,
    pub cohesion_level: f64,
    pub interaction_intensity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationFlow {
    pub from_person: String,
    pub to_person: String,
    pub communication_type: String,
    pub intensity: f64,
    pub duration: std::time::Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyImplications {
    pub privacy_risk_level: f64,
    pub sensitive_information_present: bool,
    pub recording_restrictions: Vec<String>,
    pub anonymization_requirements: Vec<String>,
}

/// Réponses par modalité
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceResponse {
    pub speech_content: String,
    pub voice_characteristics: VoiceCharacteristics,
    pub prosodic_features: ProsodicFeatures,
    pub emotional_expression: String,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceCharacteristics {
    pub pitch_range: (f64, f64),
    pub speaking_rate: f64,
    pub volume_level: f64,
    pub voice_quality: String,
    pub accent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProsodicFeatures {
    pub stress_patterns: Vec<String>,
    pub intonation_contour: Vec<f64>,
    pub pause_patterns: Vec<std::time::Duration>,
    pub rhythm_characteristics: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualResponse {
    pub visual_elements: Vec<VisualElement>,
    pub display_configuration: DisplayConfiguration,
    pub animation_sequences: Vec<AnimationSequence>,
    pub visual_effects: Vec<VisualEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualElement {
    pub element_type: String,
    pub position: (f64, f64),
    pub size: (f64, f64),
    pub color: String,
    pub opacity: f64,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfiguration {
    pub resolution: (u32, u32),
    pub refresh_rate: f32,
    pub color_depth: u8,
    pub brightness: f64,
    pub contrast: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationSequence {
    pub sequence_name: String,
    pub duration: std::time::Duration,
    pub keyframes: Vec<Keyframe>,
    pub easing_function: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyframe {
    pub time_offset: std::time::Duration,
    pub properties: HashMap<String, f64>,
    pub interpolation_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualEffect {
    pub effect_type: String,
    pub parameters: HashMap<String, f64>,
    pub duration: std::time::Duration,
    pub target_elements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HapticResponse {
    pub haptic_patterns: Vec<HapticPattern>,
    pub intensity_profile: Vec<f64>,
    pub duration: std::time::Duration,
    pub haptic_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HapticPattern {
    pub pattern_type: String,
    pub frequency: f64,
    pub amplitude: f64,
    pub duration: std::time::Duration,
    pub spatial_location: Option<(f64, f64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialResponse {
    pub spatial_elements: Vec<SpatialElement>,
    pub positioning_strategy: PositioningStrategy,
    pub interaction_affordances: Vec<InteractionAffordance>,
    pub spatial_audio: Option<SpatialAudio>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialElement {
    pub element_id: String,
    pub element_type: String,
    pub position_3d: (f64, f64, f64),
    pub orientation_3d: (f64, f64, f64),
    pub scale: (f64, f64, f64),
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositioningStrategy {
    pub strategy_type: String,
    pub reference_points: Vec<(f64, f64, f64)>,
    pub positioning_constraints: Vec<String>,
    pub optimization_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionAffordance {
    pub affordance_type: String,
    pub interaction_zone: InteractionZone,
    pub required_gestures: Vec<String>,
    pub feedback_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialAudio {
    pub audio_sources: Vec<AudioSource>,
    pub acoustic_model: AcousticModel,
    pub spatialization_technique: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSource {
    pub source_id: String,
    pub position_3d: (f64, f64, f64),
    pub audio_content: String,
    pub volume: f64,
    pub directivity_pattern: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcousticModel {
    pub room_acoustics: RoomAcoustics,
    pub sound_propagation: SoundPropagation,
    pub environmental_effects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomAcoustics {
    pub reverberation_time: f64,
    pub absorption_coefficients: HashMap<String, f64>,
    pub reflection_patterns: Vec<ReflectionPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflectionPattern {
    pub surface_type: String,
    pub reflection_coefficient: f64,
    pub delay: std::time::Duration,
    pub frequency_response: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundPropagation {
    pub propagation_model: String,
    pub attenuation_factors: HashMap<String, f64>,
    pub occlusion_effects: Vec<OcclusionEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcclusionEffect {
    pub occluding_object: String,
    pub occlusion_level: f64,
    pub frequency_filtering: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalResponse {
    pub lighting_adjustments: Vec<LightingAdjustment>,
    pub acoustic_modifications: Vec<AcousticModification>,
    pub climate_control: Option<ClimateControl>,
    pub ambient_enhancements: Vec<AmbientEnhancement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightingAdjustment {
    pub light_source_id: String,
    pub adjustment_type: String,
    pub target_value: f64,
    pub transition_duration: std::time::Duration,
    pub adjustment_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcousticModification {
    pub modification_type: String,
    pub target_frequency_range: (f64, f64),
    pub adjustment_level: f64,
    pub spatial_scope: Vec<(f64, f64, f64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateControl {
    pub temperature_adjustment: Option<f64>,
    pub humidity_adjustment: Option<f64>,
    pub air_circulation_adjustment: Option<f64>,
    pub adjustment_zones: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmbientEnhancement {
    pub enhancement_type: String,
    pub intensity: f64,
    pub duration: std::time::Duration,
    pub target_mood: String,
}

/// Système principal d'interface multimodale
pub struct MultimodalInterface {
    pub voice_processor: VoiceProcessor,
    pub vision_processor: VisionProcessor,
    pub biometric_integrator: BiometricIntegrator,
    pub spatial_computer: SpatialComputer,
    pub fusion_engine: FusionEngine,
    pub interaction_orchestrator: InteractionOrchestrator,
}

impl MultimodalInterface {
    pub fn new() -> Self {
        Self {
            voice_processor: VoiceProcessor::new(),
            vision_processor: VisionProcessor::new(),
            biometric_integrator: BiometricIntegrator::new(),
            spatial_computer: SpatialComputer::new(),
            fusion_engine: FusionEngine::new(),
            interaction_orchestrator: InteractionOrchestrator::new(),
        }
    }

    /// Traitement principal d'entrée multimodale
    pub async fn process_multimodal_input(
        &self,
        input: MultimodalInput,
        emotional_state: &EmotionalState,
        self_awareness: &SelfAwareness,
    ) -> ConsciousnessResult<MultimodalResponse> {
        let start_time = Instant::now();

        // 1. Traitement parallèle de chaque modalité
        let voice_result = if let Some(voice_data) = &input.voice_data {
            Some(self.voice_processor.process_voice(voice_data, &input.context).await?)
        } else {
            None
        };

        let vision_result = if let Some(vision_data) = &input.vision_data {
            Some(self.vision_processor.process_vision(vision_data, &input.context).await?)
        } else {
            None
        };

        let biometric_result = if let Some(biometric_data) = &input.biometric_data {
            Some(self.biometric_integrator.process_biometrics(biometric_data, &input.context).await?)
        } else {
            None
        };

        let spatial_result = if let Some(spatial_data) = &input.spatial_data {
            Some(self.spatial_computer.process_spatial(spatial_data, &input.context).await?)
        } else {
            None
        };

        // 2. Fusion des modalités
        let fusion_result = self.fusion_engine.fuse_modalities(
            voice_result.as_ref(),
            vision_result.as_ref(),
            biometric_result.as_ref(),
            spatial_result.as_ref(),
            &input.context,
        ).await?;

        // 3. Orchestration de la réponse
        let orchestrated_response = self.interaction_orchestrator.orchestrate_response(
            &fusion_result,
            emotional_state,
            self_awareness,
            &input.context,
        ).await?;

        Ok(orchestrated_response)
    }

    /// Adaptation continue basée sur le feedback
    pub async fn adapt_to_feedback(
        &mut self,
        feedback: InteractionFeedback,
        context: &InteractionContext,
    ) -> ConsciousnessResult<AdaptationResult> {
        // Implémentation de l'adaptation continue
        todo!("Implement continuous adaptation")
    }
}

/// Feedback d'interaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionFeedback {
    pub user_satisfaction: f64,
    pub modality_effectiveness: HashMap<ModalityType, f64>,
    pub technical_issues: Vec<String>,
    pub user_preferences_updates: Vec<String>,
    pub accessibility_feedback: Vec<String>,
}

/// Résultat d'adaptation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationResult {
    pub adaptations_applied: Vec<String>,
    pub expected_improvement: f64,
    pub confidence_level: f64,
    pub monitoring_requirements: Vec<String>,
}

impl Default for MultimodalInterface {
    fn default() -> Self {
        Self::new()
    }
}